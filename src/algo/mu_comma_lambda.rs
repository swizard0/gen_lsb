use std::sync::Arc;
use std::marker::PhantomData;
use par_exec::{Executor, LocalContextBuilder, ExecutorNewError};

use super::Algorithm;
use super::super::pop::individual::IndividualManager;
use super::super::pop::init::PopulationInit;
use super::super::pop::init::limited;
use super::super::pop::fit::PopulationFit;
use super::super::pop::fit::standard;
use super::super::set::{Set, SetManager};

// common policy
pub trait Policy {
    // individual config
    type Indiv;
    type IndivME: Send + 'static;
    type IndivM: IndividualManager<I = Self::Indiv, FI = Self::Fit, E = Self::IndivME>;

    // population config
    type PopSE: Send + 'static;
    type PopS: Set<T = Self::Indiv, E = Self::PopSE> + Sync + Send + 'static;
    type PopSME: Send + 'static;
    type PopSM: SetManager<S = Self::PopS, E = Self::PopSME>;

    // fitness config
    type Fit;
    type FitsE: Send + 'static;
    type Fits: Set<T = (Self::Fit, usize), E = Self::FitsE> + Send + 'static;
    type FitsME: Send + 'static;
    type FitsM: SetManager<S = Self::Fits, E = Self::FitsME>;
}

pub struct LocalContext<P> where P: Policy {
    indiv_manager: P::IndivM,
    pop_set_manager: P::PopSM,
    fits_set_manager: P::FitsM,
}

impl<P> limited::RetrievePopulationManager for LocalContext<P> where P: Policy {
    type PopM = P::PopSM;

    fn retrieve(&mut self) -> &mut Self::PopM {
        &mut self.pop_set_manager
    }
}

impl<P> limited::RetrieveIndividualManager for LocalContext<P> where P: Policy {
    type IM = P::IndivM;

    fn retrieve(&mut self) -> &mut Self::IM {
        &mut self.indiv_manager
    }
}

impl<P> standard::RetrieveFitsManager for LocalContext<P> where P: Policy {
    type FitsM = P::FitsM;

    fn retrieve(&mut self) -> &mut Self::FitsM {
        &mut self.fits_set_manager
    }
}

impl<P> standard::RetrieveIndividualManager for LocalContext<P> where P: Policy {
    type IM = P::IndivM;

    fn retrieve(&mut self) -> &mut Self::IM {
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
    type PopE = <AP::P as Policy>::PopSE;
    type Pop = <AP::P as Policy>::PopS;
    type PopSME = <AP::P as Policy>::PopSME;
    type PopSM = <AP::P as Policy>::PopSM;
}

pub struct PopFitPolicy<AP>(PhantomData<AP>) where AP: APolicy;
impl<AP> standard::Policy for PopFitPolicy<AP> where AP: APolicy {
    type LocalContext = LocalContext<AP::P>;
    type Exec = AP::Exec;
    type Indiv = <AP::P as Policy>::Indiv;
    type Fit = <AP::P as Policy>::Fit;
    type IndivME = <AP::P as Policy>::IndivME;
    type IndivM = <AP::P as Policy>::IndivM;
    type PopE = <AP::P as Policy>::PopSE;
    type Pop = <AP::P as Policy>::PopS;
    type FitsE = <AP::P as Policy>::FitsE;
    type Fits = <AP::P as Policy>::Fits;
    type FitsME = <AP::P as Policy>::FitsME;
    type FitsM = <AP::P as Policy>::FitsM;
}

pub struct MuCommaLambda<AP> where AP: APolicy {
    lc_builder: AP::LCBuilder,
    pop_init: limited::LimitedPopulationInit<PopInitPolicy<AP>>,
    pop_fit: standard::StandardPopulationFit<PopFitPolicy<AP>>,
}

impl<AP> MuCommaLambda<AP> where AP: APolicy {
    pub fn new(lc_builder: AP::LCBuilder, lambda: usize) -> MuCommaLambda<AP> {
        MuCommaLambda {
            lc_builder: lc_builder,
            pop_init: limited::LimitedPopulationInit::new(lambda),
            pop_fit: standard::StandardPopulationFit::new(),
        }
    }
}

pub enum Error<AP> where AP: APolicy {
    ExecutorStart(ExecutorNewError<<AP::Exec as Executor>::E, <AP::LCBuilder as LocalContextBuilder>::E>),
    PopulationInit(limited::ErrorP<PopInitPolicy<AP>>),
    PopulationFit(standard::ErrorP<PopFitPolicy<AP>>),

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

        let current_population = Arc::new(init_population);
        let _fit_results = try!(self.pop_fit.fit(current_population.clone(), &mut executor).map_err(|e| Error::PopulationFit(e)));

        Err(Error::Dummy)
    }
}
