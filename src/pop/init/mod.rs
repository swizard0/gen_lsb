use par_exec::Executor;

pub mod limited;

use super::super::set::Set;

pub trait PopulationInit {
    type Exec: Executor;
    type Indiv;
    type Pop: Set<T = Self::Indiv>;
    type Err;

    fn init(&self, exec: &mut Self::Exec) -> Result<Self::Pop, Self::Err>;
}
