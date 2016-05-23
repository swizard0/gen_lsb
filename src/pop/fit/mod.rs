use par_exec::Executor;

pub mod standard;

use super::individual::Individual;
use super::super::set::Set;

pub trait PopulationFit {
    type Exec: Executor;
    type Indiv: Individual;
    type Pop: Set<T = Self::Indiv>;
    type Fit;
    type Fits: Set<T = (Self::Fit, usize)>;
    type Err;

    fn fit(&self, population: &Self::Pop, exec: &mut Self::Exec) -> Result<Self::Fits, Self::Err>;
}
