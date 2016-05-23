use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError, JobExecuteError};

use super::PopulationFit;
use super::super::individual::{Individual, IndividualManager, IndividualManagerMut};
use super::super::super::set::{Set, SetManager, SetManagerMut};
use super::super::super::set::union;

pub trait Policy {
    type LocalContext ; //: IndividualManagerMut<IM = Self::IndivM> + SetManagerMut<SM = Self::PopSM> + SetManagerMut<SM = Self::FitsSM>;
    type Exec: Executor<LC = Self::LocalContext>;

    type Indiv: Individual;
    type Fit;
    type IndivME: Send + 'static;
    type IndivM: IndividualManager<I = Self::Indiv, FI = Self::Fit, E = Self::IndivME>;

    type PopE: Send + 'static;
    type Pop: Set<T = Self::Indiv, E = Self::PopE>;
    type PopSME: Send + 'static;
    type PopSM: SetManager<S = Self::Pop, E = Self::PopSME>;

    type FitsE: Send + 'static;
    type Fits: Set<T = (Self::Fit, usize), E = Self::FitsE> + Send + 'static;
    type FitsSME: Send + 'static;
    type FitsSM: SetManager<S = Self::Fits, E = Self::FitsSME>;
}
