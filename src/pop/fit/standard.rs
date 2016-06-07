use std::sync::Arc;
use std::marker::PhantomData;
use par_exec::{Executor, WorkAmount, JobIterBuild, ExecutorJobError, JobExecuteError};

use super::PopulationFit;
use super::super::individual::IndividualManager;
use super::super::super::set::{Set, SetManager};
use super::super::super::set::union;

pub trait RetrieveFitsManager {
    type FitsM;

    fn retrieve(&mut self) -> &mut Self::FitsM;
}

pub trait RetrieveIndividualManager {
    type IM;

    fn retrieve(&mut self) -> &mut Self::IM;
}

pub trait Policy {
    type LocalContext: RetrieveFitsManager<FitsM = Self::FitsM> + RetrieveIndividualManager<IM = Self::IndivM>;
    type Exec: Executor<LC = Self::LocalContext>;

    type Indiv;
    type Fit;
    type IndivME: Send + 'static;
    type IndivM: IndividualManager<I = Self::Indiv, FI = Self::Fit, E = Self::IndivME>;

    type PopE: Send + 'static;
    type Pop: Set<T = Self::Indiv, E = Self::PopE> + Sync + Send + 'static;

    type FitsE: Send + 'static;
    type Fits: Set<T = (Self::Fit, usize), E = Self::FitsE> + Send + 'static;
    type FitsME: Send + 'static;
    type FitsM: SetManager<S = Self::Fits, E = Self::FitsME>;
}

pub struct StandardPopulationFit<P> where P: Policy {
    _marker: PhantomData<P>,
}

impl<P> StandardPopulationFit<P> where P: Policy {
    pub fn new() -> StandardPopulationFit<P> {
        StandardPopulationFit {
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum FitnessError<PE, FE, FME, IME> {
    Population(PE),
    FitsSet(FE),
    FitsSetManager(FME),
    IndividualManager(IME),
}

#[derive(Debug)]
pub enum Error<ExecE, PopE, FitsE, FitsME, IndivME> {
    NoOutputFitnessValues,
    Executor(ExecutorJobError<ExecE, JobExecuteError<FitnessError<PopE, FitsE, FitsME, IndivME>, union::Error<FitsE, FitsME>>>),
}

pub type ErrorP<P> where P: Policy = Error<<P::Exec as Executor>::E, P::PopE, P::FitsE, P::FitsME, P::IndivME>;

impl<P> PopulationFit for StandardPopulationFit<P> where P: Policy {
    type Exec = P::Exec;
    type Indiv = P::Indiv;
    type Pop = P::Pop;
    type Fit = P::Fit;
    type Fits = P::Fits;
    type Err = ErrorP<P>;

    fn fit<WA>(&self, population: Arc<Self::Pop>, exec: &mut Self::Exec) -> Result<Self::Fits, Self::Err>
        where WA: WorkAmount, <Self::Exec as Executor>::JIB: JobIterBuild<WA>
    {
        let population_size = population.size();
        match exec.try_execute_job(
            WA::new(population_size),
            move |local_context, input_indices| {
                let mut fitness_results = {
                    let mut set_manager = <P::LocalContext as RetrieveFitsManager>::retrieve(local_context);
                    try!(set_manager.make_set(Some(population_size)).map_err(FitnessError::FitsSetManager))
                };
                let mut indiv_manager = <P::LocalContext as RetrieveIndividualManager>::retrieve(local_context);
                for index in input_indices {
                    let indiv = try!(population.get(index).map_err(FitnessError::Population));
                    let fitness = try!(indiv_manager.fitness(indiv).map_err(FitnessError::IndividualManager));
                    try!(fitness_results.add((fitness, index)).map_err(FitnessError::FitsSet));
                }
                Ok(fitness_results)
            },
            move |local_context, fits_a, fits_b| union::union(<P::LocalContext as RetrieveFitsManager>::retrieve(local_context), fits_a, fits_b))
        {
            Ok(None) => Err(Error::NoOutputFitnessValues),
            Ok(Some(fitness_results)) => Ok(fitness_results),
            Err(e) => Err(Error::Executor(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use par_exec::Executor;
    use par_exec::par::{ParallelExecutor, Alternately};
    use super::super::super::super::set;
    use super::super::PopulationFit;
    use super::super::super::individual::IndividualManager;
    use super::{Policy, StandardPopulationFit, RetrieveFitsManager, RetrieveIndividualManager};

    struct IndivManager;
    impl IndividualManager for IndivManager {
        type I = usize;
        type FI = f64;
        type E = ();

        fn generate(&mut self, index: usize) -> Result<Self::I, Self::E> {
            Ok(index)
        }

        fn fitness(&mut self, indiv: &Self::I) -> Result<Self::FI, Self::E> {
            Ok(1.0 / *indiv as f64)
        }
    }

    struct LocalContext {
        set_manager: set::vec::Manager<(f64, usize)>,
        indiv_manager: IndivManager,
    }

    impl RetrieveFitsManager for LocalContext {
        type FitsM = set::vec::Manager<(f64, usize)>;

        fn retrieve(&mut self) -> &mut Self::FitsM {
            &mut self.set_manager
        }
    }

    impl RetrieveIndividualManager for LocalContext {
        type IM = IndivManager;

        fn retrieve(&mut self) -> &mut Self::IM {
            &mut self.indiv_manager
        }
    }

    struct TestPolicy;
    impl Policy for TestPolicy {
        type LocalContext = LocalContext;
        type Exec = ParallelExecutor<LocalContext>;

        type Indiv = usize;
        type Fit = f64;
        type IndivME = ();
        type IndivM = IndivManager;

        type PopE = set::vec::Error;
        type Pop = Vec<usize>;

        type FitsE = set::vec::Error;
        type Fits = Vec<(f64, usize)>;
        type FitsME = ();
        type FitsM = set::vec::Manager<(f64, usize)>;
    }

    #[test]
    fn parallel_fitness() {
        let exec: ParallelExecutor<_> = Default::default();
        let mut exec = exec.start(|| LocalContext {
            set_manager: set::vec::Manager::new(),
            indiv_manager: IndivManager,
        }).unwrap();

        use std::sync::Arc;
        let population = Arc::new((0 .. 1024).collect::<Vec<_>>());

        let fitness_calculator: StandardPopulationFit<TestPolicy> =
            StandardPopulationFit::new();
        let mut fit_results =
            fitness_calculator.fit::<Alternately>(population.clone(), &mut exec).unwrap();
        fit_results.sort_by_key(|v| v.1);
        assert_eq!(Arc::new(fit_results.into_iter().map(|(_, i)| i).collect::<Vec<_>>()), population);
    }
}
