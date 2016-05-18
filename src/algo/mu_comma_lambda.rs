use std::marker::PhantomData;

use par_exec::Executor;
use super::Algorithm;
use super::super::pop::PopulationInit;
use super::super::pop::individual::Individual;

pub trait Policy {
    type Exec: Executor;
    type Indiv: Individual;
    type PopInit: PopulationInit;
}

pub struct MuCommaLambda<P> where P: Policy {
    _marker: PhantomData<P>,
}

impl<P> MuCommaLambda<P> where P: Policy {
    pub fn new() -> MuCommaLambda<P> {
        MuCommaLambda {
            _marker: PhantomData,
        }
    }
}

// pub enum Error<P> where P: Policy {
pub enum Error {
    Dummy
}

impl<P> Algorithm for MuCommaLambda<P> where P: Policy {
    type Exec = P::Exec;
    type Res = P::Indiv;
    type Err = Error;

    fn run(_executor: &mut Self::Exec) -> Result<Self::Res, Self::Err> {
        Err(Error::Dummy)
    }
}
