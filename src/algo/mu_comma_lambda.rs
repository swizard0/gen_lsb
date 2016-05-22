use std::marker::PhantomData;
use par_exec::{Executor, LocalContextBuilder, ExecutorNewError};

use super::Algorithm;
use super::super::pop::individual::{Individual, IndividualManager, IndividualManagerMut};
use super::super::pop::init::PopulationInit;
use super::super::pop::init::limited;
use super::super::set::{Set, SetManager, SetManagerMut};

// common policy
pub trait Policy {
    // individual config
    type Indiv: Individual;
    type IndivME: Send + 'static;
    type IndivM: IndividualManager<I = Self::Indiv, E = Self::IndivME>;

    // init population config
    type PopInitSE: Send + 'static;
    type PopInitS: Set<T = Self::Indiv, E = Self::PopInitSE> + Send + 'static;
    type PopInitSME: Send + 'static;
    type PopInitSM: SetManager<S = Self::PopInitS, E = Self::PopInitSME>;
}

pub struct LocalContext<P> where P: Policy {
    indiv_manager: P::IndivM,
    pop_init_set_manager: P::PopInitSM,
}

impl<P> SetManagerMut for LocalContext<P> where P: Policy {
    type SM = P::PopInitSM;

    fn set_manager_mut(&mut self) -> &mut Self::SM {
        &mut self.pop_init_set_manager
    }
}

impl<P> IndividualManagerMut for LocalContext<P> where P: Policy {
    type IM = P::IndivM;

    fn individual_manager_mut(&mut self) -> &mut Self::IM {
        &mut self.indiv_manager
    }
}

// algorithm policy
pub trait APolicy {
    type P: Policy;
    type LCBuilder: LocalContextBuilder<LC = LocalContext<Self::P>>;
    type Exec: Executor<LC = LocalContext<Self::P>>;
}

pub struct PopInitPolicy<AP>(PhantomData<AP>) where AP: APolicy;

impl<AP> limited::Policy for PopInitPolicy<AP> where AP: APolicy {
    type LocalContext = LocalContext<AP::P>;
    type Exec = AP::Exec;
    type Indiv = <AP::P as Policy>::Indiv;
    type IndivME = <AP::P as Policy>::IndivME;
    type IndivM = <AP::P as Policy>::IndivM;
    type PopE = <AP::P as Policy>::PopInitSE;
    type Pop = <AP::P as Policy>::PopInitS;
    type PopSME = <AP::P as Policy>::PopInitSME;
    type PopSM = <AP::P as Policy>::PopInitSM;
}

pub struct MuCommaLambda<AP> where AP: APolicy {
    lc_builder: AP::LCBuilder,
    pop_init: limited::LimitedPopulationInit<PopInitPolicy<AP>>,
}

impl<AP> MuCommaLambda<AP> where AP: APolicy {
    pub fn new(lc_builder: AP::LCBuilder, lambda: usize) -> MuCommaLambda<AP> {
        MuCommaLambda {
            lc_builder: lc_builder,
            pop_init: limited::LimitedPopulationInit::new(lambda),
        }
    }
}

pub enum Error<AP> where AP: APolicy {
    ExecutorStart(ExecutorNewError<<AP::Exec as Executor>::E, <AP::LCBuilder as LocalContextBuilder>::E>),
    PopulationInit(limited::Error<PopInitPolicy<AP>>),

    Dummy
}

impl<AP> Algorithm for MuCommaLambda<AP> where AP: APolicy {
    type Exec = AP::Exec;
    type Res = <AP::P as Policy>::Indiv;
    type Err = Error<AP>;

    fn run(self, not_started_executor: Self::Exec) -> Result<Self::Res, Self::Err> {
        let mut executor =
            try!(not_started_executor.try_start(self.lc_builder).map_err(|e| Error::ExecutorStart(e)));
        let init_population = try!(self.pop_init.init(&mut executor).map_err(|e| Error::PopulationInit(e)));


        Err(Error::Dummy)
    }
}
