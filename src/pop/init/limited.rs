use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError, JobExecuteError};

use super::PopulationInit;
use super::super::individual::{Individual, IndividualManager, IndividualManagerMut};
use super::super::super::set::{Set, SetManager, SetManagerMut};
use super::super::super::set::union;

pub trait Policy {
    type LocalContext: SetManagerMut<SM = Self::PopSM> + IndividualManagerMut<IM = Self::IndivM>;
    type Exec: Executor<LC = Self::LocalContext>;
    type Indiv: Individual;
    type IndivM: IndividualManager<I = Self::Indiv>;
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
            move |local_context, input_indices| {
                let mut set_manager = local_context.set_manager_mut();
                for _ in input_indices {
                }
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
