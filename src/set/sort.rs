use par_exec::{Executor, WorkAmount, JobIterBuild, ExecutorJobError, JobExecuteError};
use super::{Set, SetManager, merge};

pub trait SortManager {
    type S: Set;
    type E;

    fn sort<SF>(&mut self, set: &mut Self::S, pred: SF) -> Result<(), Self::E> where SF: Fn(usize, usize) -> bool;
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
    Executor(ExecutorJobError<ExecE, JobExecuteError<SortError<SE, SME, SRE>, merge::Error<SE, SME>>>),
}

pub fn sort<Exec, LC, S, SetM, SortM, WA, F>(amount: WA, pred: F, exec: &mut Exec) -> Result<S, Error<Exec::E, S::E, SetM::E, SortM::E>> where
    LC: RetrieveSortManager<SortM = SortM> + RetrieveSetManager<SetM = SetM>,
    Exec: Executor<LC = LC>,
    S: Set<T = usize> + Send + Sync + 'static,
    SetM: SetManager<S = S>,
    SortM: SortManager<S = S>,
    S::E: Send + 'static,
    SetM::E: Send + 'static,
    SortM::E: Send + 'static,
    WA: WorkAmount,
    Exec::JIB: JobIterBuild<WA>,
    F: Fn(usize, usize) -> bool + Sync + Send + 'static
{
    use std::sync::Arc;
    let map_pred = Arc::new(pred);
    let reduce_pred = map_pred.clone();

    match exec.try_execute_job(
        amount,
        move |local_context, input_indices| {
            let mut sort_chunk = {
                let mut set_manager = <LC as RetrieveSetManager>::retrieve(local_context);
                try!(set_manager.make_set(None).map_err(SortError::SetManager))
            };
            for index in input_indices {
                try!(sort_chunk.add(index).map_err(SortError::Set));
            }
            let mut sort_manager = <LC as RetrieveSortManager>::retrieve(local_context);
            try!(sort_manager.sort(&mut sort_chunk, |a, b| map_pred(a, b)).map_err(SortError::Sort));
            Ok(sort_chunk)
        },
        move |local_context, set_a, set_b| merge::merge(<LC as RetrieveSetManager>::retrieve(local_context), set_a, set_b, |&a, &b| reduce_pred(a, b)))
    {
        Ok(None) => Err(Error::EmptySet),
        Ok(Some(sorted_set)) => Ok(sorted_set),
        Err(e) => Err(Error::Executor(e)),
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::sync::Arc;
    use self::rand::Rng;
    use super::{RetrieveSortManager, RetrieveSetManager, sort};
    use super::super::vec;
    use par_exec::{Executor, WorkAmount};
    use par_exec::par::{ParallelExecutor, ByEqualChunks};

    struct LocalContext(vec::Manager<usize>);

    impl RetrieveSortManager for LocalContext {
        type SortM = vec::Manager<usize>;

        fn retrieve(&mut self) -> &mut Self::SortM {
            &mut self.0
        }
    }

    impl RetrieveSetManager for LocalContext {
        type SetM = vec::Manager<usize>;

        fn retrieve(&mut self) -> &mut Self::SetM {
            &mut self.0
        }
    }

    #[test]
    fn parallel_sort() {
        let total = 131072;
        let mut rng = rand::thread_rng();
        let vec: Arc<Vec<u64>> = Arc::new((0 .. total).map(|_| rng.gen()).collect());

        let exec: ParallelExecutor<_> = Default::default();
        let mut exec = exec.start(|| LocalContext(vec::Manager::new())).unwrap();

        let vec_clone = vec.clone();
        let sorted_indices = sort(
            ByEqualChunks::new(vec.len()),
            move |ia, ib| vec_clone[ia] < vec_clone[ib],
            &mut exec).unwrap();

        assert_eq!(sorted_indices.len(), total);
        for i in 1 .. total {
            assert!(vec[sorted_indices[i - 1]] <= vec[sorted_indices[i]]);
        }
    }
}
