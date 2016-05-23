use std::sync::Arc;
use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError, JobExecuteError};

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

    fn fit(&self, population: Arc<Self::Pop>, exec: &mut Self::Exec) -> Result<Self::Fits, Self::Err> {
        let population_size = population.size();
        match exec.try_execute_job(
            population_size,
            move |local_context, input_indices| {
                let mut fitness_results = {
                    let mut set_manager = <P::LocalContext as RetrieveFitsManager>::retrieve(local_context);
                    try!(set_manager.make_set(Some(population_size)).map_err(|e| FitnessError::FitsSetManager(e)))
                };
                let mut indiv_manager = <P::LocalContext as RetrieveIndividualManager>::retrieve(local_context);
                for index in input_indices {
                    let indiv = try!(population.get(index).map_err(|e| FitnessError::Population(e)));
                    let fitness = try!(indiv_manager.fitness(indiv).map_err(|e| FitnessError::IndividualManager(e)));
                    try!(fitness_results.add((fitness, index)).map_err(|e| FitnessError::FitsSet(e)));
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
    use par_exec::par::ParallelExecutor;
    use super::super::super::super::set;
    use super::super::PopulationFit;
    use super::super::super::individual::{Individual, IndividualManager};
    use super::{Policy, StandardPopulationFit, RetrieveFitsManager, RetrieveIndividualManager};

}
