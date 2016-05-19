use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError};

use super::PopulationInit;
use super::super::individual::Individual;
use super::super::super::set::{Set, SetManager};
use super::super::super::set::union::SetsUnion;

pub trait Policy {
    type LocalContext;
    type Exec: Executor<LC = Self::LocalContext>;
    type Indiv: Individual;
    type Pop: Set<T = Self::Indiv>;
    type PopSM: SetManager<S = Self::Pop>;
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
    Executor(<P::Exec as Executor>::E),
    Several(Vec<Error<P>>),
}

impl<P> PopulationInit for LimitedPopulationInit<P> where P: Policy {
    type Exec = P::Exec;
    type Indiv = P::Indiv;
    type Pop = P::Pop;
    type Err = Error<P>;

    fn init(&self, exec: &mut Self::Exec) -> Result<Self::Pop, Self::Err> {
        struct InitJob<P>(PhantomData<P>);

        // impl<P> Job for InitJob<P> {
        //     type LC = P::Exec::LC;
        //     type R = P::Pop;
        //     type RR = ?? SetsUnion<P::PopSM>    : Reducer<R = Self::R> + ReducerRetrieve<LC = Self::LC>;
        //     type E = Error<P>;

        //     fn execute<IS>(&self, local_context: &mut Self::LC, input_indices: IS) ->
        //         Result<Self::R, JobExecuteError<Self::E, <Self::RR as Reducer>::E>>
        //         where IS: Iterator<Item = usize>;
        // }

        // fn err_map_rec(err: ExecutorJobError<P::Exec::E, ?>) -> Error<P> {
        //     match err {
        //         ExecutorJobError::Executor(e) =>
        //             Error::Executor(e),
        //         ExecutorJobError::Job(e) =>
        //             unimplemented!(),
        //         ExecutorJobError::Several(ee) =>
        //             Error::Several(ee.into_iter().map(err_map_rec).collect()),
        //     }
        // }

        // match exec.execute_job(self.limit, init_job) {
        //     Ok(None) => Err(Error::NoOutputPopulation),
        //     Ok(Some(population)) => Ok(population),
        //     Err(e) => Err(err_map_rec(e)),
        // }

        Err(Error::NoOutputPopulation)
    }
}
