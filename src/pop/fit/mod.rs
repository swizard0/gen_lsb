use std::sync::Arc;
use par_exec::Executor;

pub mod standard;

use super::super::set::Set;

pub trait PopulationFit {
    type Exec: Executor;
    type Indiv;
    type Pop: Set<T = Self::Indiv>;
    type Fit;
    type Fits: Set<T = (Self::Fit, usize)>;
    type Err;

    fn fit(&self, population: Arc<Self::Pop>, exec: &mut Self::Exec) -> Result<Self::Fits, Self::Err>;
}
