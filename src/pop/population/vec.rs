
use super::Population;
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
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::super::Population;
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
    }

    #[test]
    fn basic() {
        run_basic(Vec::new());
    }
}
