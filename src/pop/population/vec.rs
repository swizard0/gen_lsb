
use super::{Population, PopulationEmpty};
use super::super::individual::Individual;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    IndexOutOfRange { index: usize, total: usize },
}

impl<T> Population for Vec<T> where T: Individual {
    type I = T;
    type E = Error;

    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Result<&Self::I, Self::E> {
        let slice: &[T] = self;
        slice.get(index).ok_or(Error::IndexOutOfRange { index: index, total: self.len(), })
    }

    fn add(&mut self, indiv: Self::I) -> Result<(), Self::E> {
        self.push(indiv);
        Ok(())
    }

    fn del(&mut self, index: usize) -> Result<Self::I, Self::E> {
        if index < self.len() {
            Ok(self.swap_remove(index))
        } else {
            Err(Error::IndexOutOfRange { index: index, total: self.len(), })
        }
    }

    fn merge(mut self, other: Self) -> Result<Self, Self::E> {
        self.extend(other.into_iter());
        Ok(self)
    }
}

impl<T> PopulationEmpty for Vec<T> where T: Individual {
    type E = Error;

    fn make_empty() -> Result<Self, Self::E> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::super::{Population, PopulationEmpty};
    use super::super::super::individual;

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

    fn run_basic<P>(mut pop: P) where P: Population<I = TestI, E = Error> {
        assert_eq!(pop.size(), 0);
        assert_eq!(pop.add(TestI(TestC(0))), Ok(()));
        assert_eq!(pop.size(), 1);
        assert_eq!(pop.add(TestI(TestC(1))), Ok(()));
        assert_eq!(pop.size(), 2);
        assert_eq!(pop.get(0), Ok(&TestI(TestC(0))));
        assert_eq!(pop.get(1), Ok(&TestI(TestC(1))));
        assert_eq!(pop.get(2), Err(Error::IndexOutOfRange { index: 2, total: 2, }));
        assert_eq!(pop.del(0), Ok(TestI(TestC(0))));
        assert_eq!(pop.size(), 1);
        assert_eq!(pop.get(0), Ok(&TestI(TestC(1))));
    }

    #[test]
    fn basic() {
        let pop: Vec<_> = PopulationEmpty::make_empty().unwrap();
        run_basic(pop);
    }

    #[test]
    fn merge() {
        let v1 = vec![TestI(TestC(1)), TestI(TestC(2)), TestI(TestC(3))];
        let v2 = vec![TestI(TestC(4)), TestI(TestC(5))];
        assert_eq!(Population::merge_many(vec![v1, v2].into_iter()),
                   Ok(Some(vec![TestI(TestC(1)), TestI(TestC(2)), TestI(TestC(3)), TestI(TestC(4)), TestI(TestC(5))])));
        let empty: Vec<Vec<TestI>> = Vec::new();
        assert_eq!(Population::merge_many(empty.into_iter()), Ok(None));
    }
}
