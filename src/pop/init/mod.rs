use par_exec::{Executor, WorkAmount, JobIterBuild};

pub mod limited;

use super::super::set::Set;

pub trait PopulationInit {
    type Exec: Executor;
    type Indiv;
    type Pop: Set<T = Self::Indiv>;
    type Err;

    fn init<WA>(&self, exec: &mut Self::Exec) -> Result<Self::Pop, Self::Err>
        where WA: WorkAmount, <Self::Exec as Executor>::JIB: JobIterBuild<WA>;
}
