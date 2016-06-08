use std::vec::IntoIter;
use std::marker::PhantomData;
use super::{Set, SetManager};
use super::sort::SortManager;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    IndexOutOfRange { index: usize, total: usize },
}

pub struct VecSetIter<T, E> {
    iter: Option<IntoIter<T>>,
    _marker: PhantomData<E>,
}

impl<T, E> Iterator for VecSetIter<T, E> {
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut iter) = self.iter.take() {
            match iter.next() {
                None => None,
                Some(value) => {
                    self.iter = Some(iter);
                    Some(Ok(value))
                }
            }
        } else {
            None
        }
    }
}

impl<T> Set for Vec<T> where T: Sized {
    type T = T;
    type E = Error;
    type I = VecSetIter<Self::T, Self::E>;

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

    fn into_iter(self) -> Self::I {
        VecSetIter {
            iter: Some(IntoIterator::into_iter(self)),
            _marker: PhantomData,
        }
    }
}

pub struct Manager<T> {
    _marker: PhantomData<T>,
}

impl<T> Manager<T> {
    pub fn new() -> Manager<T> {
        Manager {
            _marker: PhantomData,
        }
    }
}

impl<T> SetManager for Manager<T> {
    type S = Vec<T>;
    type E = ();

    fn make_set(&mut self, size_hint: Option<usize>) -> Result<Self::S, Self::E> {
        Ok(match size_hint {
            Some(hint) => Vec::with_capacity(hint),
            None => Vec::new(),
        })
    }

    fn reserve(&mut self, set: &mut Self::S, additional: usize) -> Result<(), Self::E> {
        Ok(set.reserve(additional))
    }
}

impl SortManager for Manager<usize> {
    type S = Vec<usize>;
    type E = ();

    fn sort<SF>(&mut self, set: &mut Self::S, pred: SF) -> Result<(), Self::E> where SF: Fn(usize, usize) -> bool {
        use std::cmp::Ordering;
        set.sort_by(|&a, &b| if pred(a, b) {
            Ordering::Less
        } else {
            Ordering::Greater
        });
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::{Error, Manager};
    use super::super::{Set, SetManager};

    fn run_basic<S>(mut set: S) where S: Set<T = u8, E = Error> {
        assert_eq!(set.size(), 0);
        assert_eq!(set.add(0), Ok(()));
        assert_eq!(set.size(), 1);
        assert_eq!(set.add(1), Ok(()));
        assert_eq!(set.size(), 2);
        assert_eq!(set.get(0), Ok(&0));
        assert_eq!(set.get(1), Ok(&1));
        assert_eq!(set.get(2), Err(Error::IndexOutOfRange { index: 2, total: 2, }));
        assert_eq!(set.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>(), vec![0, 1]);
    }

    #[test]
    fn basic() {
        run_basic(Manager::new().make_set(None).unwrap());
    }
}
