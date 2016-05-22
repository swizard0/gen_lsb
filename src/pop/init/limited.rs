use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError, JobExecuteError};

use super::PopulationInit;
use super::super::individual::Individual;
use super::super::super::set::{Set, SetManager};
use super::super::super::set::union;

pub trait Policy {
    type LocalContext: AsMut<Self::PopSM>;
    type Exec: Executor<LC = Self::LocalContext>;
    type Indiv: Individual;
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

pub enum Error<P> where P: Policy {
    NoOutputPopulation,
    Executor(ExecutorJobError<<P::Exec as Executor>::E, JobExecuteError<(), union::Error<P::PopE, P::PopSME>>>),
}

impl<P> PopulationInit for LimitedPopulationInit<P> where P: Policy {
    type Exec = P::Exec;
    type Indiv = P::Indiv;
    type Pop = P::Pop;
    type Err = Error<P>;

    fn init(&self, exec: &mut Self::Exec) -> Result<Self::Pop, Self::Err> {
        match exec.try_execute_job(
            self.limit,
            move |_local_context, _input_indices| {
                Err(())
            },
            move |local_context, pop_a, pop_b| union::union(local_context, pop_a, pop_b))
        {
            Ok(None) => Err(Error::NoOutputPopulation),
            Ok(Some(population)) => Ok(population),
            Err(e) => Err(Error::Executor(e)),
        }
    }
}
