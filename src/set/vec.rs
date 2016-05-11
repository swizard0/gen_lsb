
use super::{Set, SetEmpty};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    IndexOutOfRange { index: usize, total: usize },
}

impl<T> Set for Vec<T> where T: Sized + Sync + Send {
    type T = T;
    type E = Error;

    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Result<&Self::T, Self::E> {
        let slice: &[T] = self;
        slice.get(index).ok_or(Error::IndexOutOfRange { index: index, total: self.len(), })
    }

    fn add(&mut self, item: Self::T) -> Result<(), Self::E> {
        self.push(item);
        Ok(())
    }

    fn del(&mut self, index: usize) -> Result<Self::T, Self::E> {
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

impl<T> SetEmpty for Vec<T> where T: Sized + Sync + Send {
    type E = Error;

    fn make_empty() -> Result<Self, Self::E> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::super::{Set, SetEmpty};

    fn run_basic<S>(mut set: S) where S: Set<T = u8, E = Error> {
        assert_eq!(set.size(), 0);
        assert_eq!(set.add(0), Ok(()));
        assert_eq!(set.size(), 1);
        assert_eq!(set.add(1), Ok(()));
        assert_eq!(set.size(), 2);
        assert_eq!(set.get(0), Ok(&0));
        assert_eq!(set.get(1), Ok(&1));
        assert_eq!(set.get(2), Err(Error::IndexOutOfRange { index: 2, total: 2, }));
        assert_eq!(set.del(0), Ok(0));
        assert_eq!(set.size(), 1);
        assert_eq!(set.get(0), Ok(&1));
    }

    #[test]
    fn basic() {
        let set: Vec<_> = SetEmpty::make_empty().unwrap();
        run_basic(set);
    }

    #[test]
    fn merge() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        assert_eq!(Set::merge_many(vec![v1, v2].into_iter()), Ok(Some(vec![1, 2, 3, 4, 5])));
        let empty: Vec<Vec<u8>> = Vec::new();
        assert_eq!(Set::merge_many(empty.into_iter()), Ok(None));
    }
}
