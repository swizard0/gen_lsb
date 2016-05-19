use std::marker::PhantomData;
use par_exec::{Executor, LocalContextBuilder, ExecutorNewError};

use super::Algorithm;
use super::super::pop::individual::Individual;
use super::super::pop::init::PopulationInit;
use super::super::pop::init::limited;
use super::super::set::{Set, SetManager};

pub trait LCPolicy {
    type Indiv: Individual;
    type PopInitSet: Set<T = Self::Indiv>;
    type PopInitSetManager: SetManager<S = Self::PopInitSet>;
}

pub struct LocalContext<LCP> where LCP: LCPolicy {
    pop_init_set_manager: LCP::PopInitSetManager,
}

impl<LCP> AsMut<LCP::PopInitSetManager> for LocalContext<LCP> where LCP: LCPolicy {
    fn as_mut(&mut self) -> &mut LCP::PopInitSetManager {
        &mut self.pop_init_set_manager
    }
}

pub trait Policy {
    type LCP: LCPolicy;
    type LCBuilder: LocalContextBuilder<LC = LocalContext<Self::LCP>>;
    type Exec: Executor<LC = LocalContext<Self::LCP>>;
}

pub struct LPIPolicy<P>(PhantomData<P>) where P: Policy;

impl<P> limited::Policy for LPIPolicy<P> where P: Policy {
    type LocalContext = LocalContext<P::LCP>;
    type Exec = P::Exec;
    type Indiv = <P::LCP as LCPolicy>::Indiv;
    type Pop = <<P::LCP as LCPolicy>::PopInitSetManager as SetManager>::S;
    type PopSM = <P::LCP as LCPolicy>::PopInitSetManager;
}

pub struct MuCommaLambda<P> where P: Policy {
    lc_builder: P::LCBuilder,
    pop_init: limited::LimitedPopulationInit<LPIPolicy<P>>,
}

impl<P> MuCommaLambda<P> where P: Policy {
    pub fn new(lc_builder: P::LCBuilder, lambda: usize) -> MuCommaLambda<P> {
        MuCommaLambda {
            lc_builder: lc_builder,
            pop_init: limited::LimitedPopulationInit::new(lambda),
        }
    }
}

pub enum Error<P> where P: Policy {
    ExecutorStart(ExecutorNewError<<P::Exec as Executor>::E, <P::LCBuilder as LocalContextBuilder>::E>),
    PopulationInit(limited::Error<LPIPolicy<P>>),

    Dummy
}

impl<P> Algorithm for MuCommaLambda<P> where P: Policy {
    type Exec = P::Exec;
    type Res = <P::LCP as LCPolicy>::Indiv;
    type Err = Error<P>;

    fn run(self, not_started_executor: Self::Exec) -> Result<Self::Res, Self::Err> {
        let mut executor =
            try!(not_started_executor.start(self.lc_builder).map_err(|e| Error::ExecutorStart(e)));
        let init_population = try!(self.pop_init.init(&mut executor).map_err(|e| Error::PopulationInit(e)));


        Err(Error::Dummy)
    }
}
