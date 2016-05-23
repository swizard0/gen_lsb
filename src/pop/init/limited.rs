use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError, JobExecuteError};

use super::PopulationInit;
use super::super::individual::{Individual, IndividualManager};
use super::super::super::set::{Set, SetManager};
use super::super::super::set::union;

pub trait RetrievePopulationManager {
    type PopM;

    fn retrieve(&mut self) -> &mut Self::PopM;
}

pub trait RetrieveIndividualManager {
    type IM;

    fn retrieve(&mut self) -> &mut Self::IM;
}

pub trait Policy {
    type LocalContext: RetrievePopulationManager<PopM = Self::PopSM> + RetrieveIndividualManager<IM = Self::IndivM>;
    type Exec: Executor<LC = Self::LocalContext>;
    type Indiv: Individual;
    type IndivME: Send + 'static;
    type IndivM: IndividualManager<I = Self::Indiv, E = Self::IndivME>;
    type PopE: Send + 'static;
    type Pop: Set<T = Self::Indiv, E = Self::PopE> + Send + 'static;
    type PopSME: Send + 'static;
    type PopSM: SetManager<S = Self::Pop, E = Self::PopSME>;
}

pub struct LimitedPopulationInit<P> where P: Policy {
    limit: usize,
    _marker: PhantomData<P>,
}

impl<P> LimitedPopulationInit<P> where P: Policy {
    pub fn new(limit: usize) -> LimitedPopulationInit<P> {
        LimitedPopulationInit {
            limit: limit,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum GenerateError<SE, SME, IME> {
    Set(SE),
    SetManager(SME),
    IndividualManager(IME),
}

#[derive(Debug)]
pub enum Error<ExecE, PopE, PopSME, IndivME> {
    NoOutputPopulation,
    Executor(ExecutorJobError<ExecE, JobExecuteError<GenerateError<PopE, PopSME, IndivME>, union::Error<PopE, PopSME>>>),
}

pub type ErrorP<P> where P: Policy = Error<<P::Exec as Executor>::E, P::PopE, P::PopSME, P::IndivME>;

impl<P> PopulationInit for LimitedPopulationInit<P> where P: Policy {
    type Exec = P::Exec;
    type Indiv = P::Indiv;
    type Pop = P::Pop;
    type Err = ErrorP<P>;

    fn init(&self, exec: &mut Self::Exec) -> Result<Self::Pop, Self::Err> {
        match exec.try_execute_job(
            self.limit,
            move |local_context, input_indices| {
                let mut population = {
                    let mut set_manager = <P::LocalContext as RetrievePopulationManager>::retrieve(local_context);
                    try!(set_manager.make_set(None).map_err(|e| GenerateError::SetManager(e)))
                };
                let mut indiv_manager = <P::LocalContext as RetrieveIndividualManager>::retrieve(local_context);
                for index in input_indices {
                    let indiv = try!(indiv_manager.generate(index).map_err(|e| GenerateError::IndividualManager(e)));
                    try!(population.add(indiv).map_err(|e| GenerateError::Set(e)));
                }
                Ok(population)
            },
            move |local_context, pop_a, pop_b| union::union(<P::LocalContext as RetrievePopulationManager>::retrieve(local_context), pop_a, pop_b))
        {
            Ok(None) => Err(Error::NoOutputPopulation),
            Ok(Some(population)) => Ok(population),
            Err(e) => Err(Error::Executor(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use par_exec::Executor;
    use par_exec::par::ParallelExecutor;
    use super::super::super::super::set;
    use super::super::PopulationInit;
    use super::super::super::individual::{Individual, IndividualManager};
    use super::{Policy, LimitedPopulationInit, RetrievePopulationManager, RetrieveIndividualManager};

    #[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
    struct Indiv(usize);
    impl Individual for Indiv {}

    struct IndivManager;
    impl IndividualManager for IndivManager {
        type I = Indiv;
        type FI = ();
        type E = ();

        fn generate(&mut self, index: usize) -> Result<Self::I, Self::E> {
            Ok(Indiv(index))
        }

        fn fitness(&mut self, _indiv: &Self::I) -> Result<Self::FI, Self::E> {
            Ok(())
        }
    }

    struct LocalContext {
        set_manager: set::vec::Manager<Indiv>,
        indiv_manager: IndivManager,
    }

    impl RetrievePopulationManager for LocalContext {
        type PopM = set::vec::Manager<Indiv>;

        fn retrieve(&mut self) -> &mut Self::PopM {
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
        type Indiv = Indiv;
        type IndivME = ();
        type IndivM = IndivManager;
        type PopE = set::vec::Error;
        type Pop = Vec<Indiv>;
        type PopSME = ();
        type PopSM = set::vec::Manager<Indiv>;
    }

    #[test]
    fn parallel_generator() {
        let exec: ParallelExecutor<_> = Default::default();
        let mut exec = exec.start(|| LocalContext {
            set_manager: set::vec::Manager::new(),
            indiv_manager: IndivManager,
        }).unwrap();

        let initializer: LimitedPopulationInit<TestPolicy> =
            LimitedPopulationInit::new(1024);
        let mut population = initializer.init(&mut exec).unwrap();
        population.sort();
        assert_eq!(population, (0 .. 1024).map(|i| Indiv(i)).collect::<Vec<_>>());
    }
}
