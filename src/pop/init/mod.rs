use par_exec::Executor;

// pub mod limited;

use super::individual::Individual;
use super::super::set::Set;

pub trait PopulationInit {
    type Exec: Executor;
    type Indiv: Individual;
    type Pop: Set<T = Self::Indiv>;
    type Err;

    fn init(&self, exec: &mut Self::Exec) -> Result<Self::Pop, Self::Err>;
}
