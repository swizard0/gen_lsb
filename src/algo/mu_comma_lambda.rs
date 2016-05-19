use std::marker::PhantomData;

use par_exec::Executor;
use super::Algorithm;
use super::super::pop::init::PopulationInit;
use super::super::pop::individual::Individual;

pub trait Policy {
    type Exec: Executor;
    type Indiv: Individual;
    type PopInit: PopulationInit<Exec = Self::Exec>;
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

pub enum Error<P> where P: Policy {
    PopulationInit(<P::PopInit as PopulationInit>::Err),

    Dummy
}

impl<P> Algorithm for MuCommaLambda<P> where P: Policy {
    type Exec = P::Exec;
    type Res = P::Indiv;
    type Err = Error<P>;

    fn run(mut executor: Self::Exec) -> Result<Self::Res, Self::Err> {
        // let init_population = try!(P::PopInit::init(executor).map_err(|e| Error::PopulationInit(e)));

        Err(Error::Dummy)
    }
}
