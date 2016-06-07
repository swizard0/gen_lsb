use std::sync::Arc;
use par_exec::{Executor, WorkAmount, JobIterBuild, ExecutorJobError, JobExecuteError};
use super::{Set, SetManager, union};

pub trait SortManager {
    type S: Set;
    type E;

    fn sort(&mut self, set: &mut Self::S) -> Result<(), Self::E>;
}

pub trait RetrieveSortManager {
    type SortM;

    fn retrieve(&mut self) -> &mut Self::SortM;
}

pub trait RetrieveSetManager {
    type SetM;

    fn retrieve(&mut self) -> &mut Self::SetM;
}

#[derive(Debug)]
pub enum SortError<SE, SME, SRE> {
    Set(SE),
    SetManager(SME),
    Sort(SRE)
}

#[derive(Debug)]
pub enum Error<ExecE, SE, SME, SRE> {
    EmptySet,
    Executor(ExecutorJobError<ExecE, JobExecuteError<SortError<SE, SME, SRE>, union::Error<SE, SME>>>),
}

pub fn sort<Exec, LC, T, S, SetM, SortM, WA>(set: Arc<S>, exec: &mut Exec) -> Result<S, Error<Exec::E, S::E, SetM::E, SortM::E>> where
    LC: RetrieveSortManager<SortM = SortM> + RetrieveSetManager<SetM = SetM>,
    Exec: Executor<LC = LC>,
    T: Clone,
    S: Set<T = T> + Send + Sync + 'static,
    SetM: SetManager<S = S>,
    SortM: SortManager<S = S>,
    S::E: Send + 'static,
    SetM::E: Send + 'static,
    SortM::E: Send + 'static,
    WA: WorkAmount,
    Exec::JIB: JobIterBuild<WA>
{
    let set_size = set.size();
    match exec.try_execute_job(
        WA::new(set_size),
        move |local_context, input_indices| {
            let mut sort_chunk = {
                let mut set_manager = <LC as RetrieveSetManager>::retrieve(local_context);
                try!(set_manager.make_set(Some(set_size)).map_err(SortError::SetManager))
            };
            for index in input_indices {
                let elem = try!(set.get(index).map_err(SortError::Set));
                try!(sort_chunk.add(elem.clone()).map_err(SortError::Set));
            }
            let mut sort_manager = <LC as RetrieveSortManager>::retrieve(local_context);
            try!(sort_manager.sort(&mut sort_chunk).map_err(SortError::Sort));
            Ok(sort_chunk)
        },
        move |local_context, set_a, set_b| union::union(<LC as RetrieveSetManager>::retrieve(local_context), set_a, set_b))
    {
        Ok(None) => Err(Error::EmptySet),
        Ok(Some(sorted_set)) => Ok(sorted_set),
        Err(e) => Err(Error::Executor(e)),
    }
}
