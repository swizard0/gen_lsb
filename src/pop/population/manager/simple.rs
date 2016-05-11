use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::marker::PhantomData;
use super::PopulationJobs;
use super::super::super::super::set::{Set, SetEmpty};
use super::super::super::individual::Individual;
use super::super::super::individual::manager::IndividualManager;

pub struct SimplePopulationJobs<P, IM> {
    limit: usize,
    _marker: PhantomData<(P, IM)>,
}

impl<P, IM> SimplePopulationJobs<P, IM> {
    pub fn new(limit: usize) -> SimplePopulationJobs<P, IM> {
        SimplePopulationJobs {
            limit: limit,
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error<PE, IME> {
    InitPopulation(PE),
    InitIndividualManager(IME),
}

impl<I, P, IM, PE> PopulationJobs for SimplePopulationJobs<P, IM> where
    I: Individual,
    IM: IndividualManager<I = I>,
    P: Set<T = I, E = PE> + SetEmpty<E = PE>,
    PE: Sync + Send
{
    type I = I;
    type P = P;
    type IM = IM;
    type E = Error<PE, IM::E>;

    fn init(&self, individual_manager: &mut Self::IM, sync_counter: Arc<AtomicUsize>) -> Result<Self::P, Self::E> {
        let mut population = try!(P::make_empty().map_err(|e| Error::InitPopulation(e)));
        loop {
            let i = sync_counter.fetch_add(1, Ordering::Relaxed);
            if i >= self.limit {
                return Ok(population);
            }

            let indiv = try!(individual_manager.generate().map_err(|e| Error::InitIndividualManager(e)));
            try!(population.add(indiv).map_err(|e| Error::InitPopulation(e)));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use super::SimplePopulationJobs;
    use super::super::PopulationJobs;
    use super::super::super::super::super::set::Set;
    use super::super::super::super::individual;

    #[derive(PartialEq, Eq, Debug)]
    struct TestC(u8);

    impl individual::chromosome::Chromosome for TestC {
        type E = ();
    }

    #[derive(PartialEq, Eq, Debug)]
    struct TestI(TestC);

    impl individual::Individual for TestI {
        type C = TestC;
        type E = ();

        fn get_chromosome(&self) -> &Self::C {
            &self.0
        }
    }

    struct TestIM(u8);

    impl individual::manager::IndividualManager for TestIM {
        type I = TestI;
        type E = ();

        fn generate(&mut self) -> Result<Self::I, Self::E> {
            self.0 += 1;
            Ok(TestI(TestC(self.0)))
        }
    }

    #[test]
    fn init() {
        let jobs: SimplePopulationJobs<Vec<TestI>, TestIM> =
            SimplePopulationJobs::new(10);
        let mut im = TestIM(0);
        let p = jobs.init(&mut im, Arc::new(AtomicUsize::new(0))).unwrap();
        for i in 0 .. 10 {
            assert_eq!(Set::get(&p, i), Ok(&TestI(TestC(i as u8 + 1))));
        }
    }
}
