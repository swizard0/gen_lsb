use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError, JobExecuteError};

use super::PopulationFit;
use super::super::individual::{Individual, IndividualManager};
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

    type Indiv: Individual;
    type Fit;
    type IndivME: Send + 'static;
    type IndivM: IndividualManager<I = Self::Indiv, FI = Self::Fit, E = Self::IndivME>;

    type PopE: Send + 'static;
    type Pop: Set<T = Self::Indiv, E = Self::PopE>;

    type FitsE: Send + 'static;
    type Fits: Set<T = (Self::Fit, usize), E = Self::FitsE> + Send + 'static;
    type FitsME: Send + 'static;
    type FitsM: SetManager<S = Self::Fits, E = Self::FitsME>;
}
