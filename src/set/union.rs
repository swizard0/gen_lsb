use par_exec::Reducer;
use super::{Set, SetManager};

#[derive(PartialEq, Debug)]
pub enum Error<ES, ESM> {
    Set(ES),
    SetManager(ESM),
}

pub struct SetsUnion<SM> {
    set_manager: SM,
}

impl<SM> SetsUnion<SM> {
    pub fn new(set_manager: SM) -> SetsUnion<SM> {
        SetsUnion {
            set_manager: set_manager,
        }
    }
}

impl<S, T, ES, SM, ESM> Reducer for SetsUnion<SM> where
    T: PartialOrd,
    S: Set<T = T, E = ES>,
    SM: SetManager<S = S, E = ESM>
{
    type R = S;
    type E = Error<ES, ESM>;

    fn len(&self, item: &Self::R) -> Option<usize> {
        Some(item.size())
    }

    fn reduce(&mut self, mut item_a: Self::R, item_b: Self::R) -> Result<Self::R, Self::E> {
        try!(self.set_manager.reserve(&mut item_a, item_b.size()).map_err(|e| Error::SetManager(e)));
        for maybe_value in item_b.into_iter() {
            let value = try!(maybe_value.map_err(|e| Error::Set(e)));
            try!(item_a.add(value).map_err(|e| Error::Set(e)));
        }
        Ok(item_a)
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::collections::HashSet;
    use par_exec::Reducer;
    use self::rand::Rng;
    use super::{Error, SetsUnion};
    use super::super::SetManager;
    use super::super::vec::Manager;

    #[test]
    fn union_sorted_vecs() {
        let mut rng = rand::thread_rng();
        let vec_a: Vec<u64> = (0 .. 1024).map(|_| rng.gen()).collect();
        let vec_b: Vec<u64> = (0 .. 768).map(|_| rng.gen()).collect();
        let vec_a_clone = vec_a.clone();
        let vec_b_clone = vec_b.clone();

        let mut sets_unioner = SetsUnion::new(Manager::new());

        assert_eq!(sets_unioner.len(&vec_a), Some(1024));
        assert_eq!(sets_unioner.len(&vec_b), Some(768));

        let vec_c = sets_unioner.reduce(vec_a, vec_b).unwrap();
        assert_eq!(sets_unioner.len(&vec_c), Some(1024 + 768));

        let table: HashSet<u64> = vec_c.into_iter().collect();

        for value in vec_a_clone.into_iter().chain(vec_b_clone.into_iter()) {
            assert!(table.contains(&value));
        }
    }

    #[test]
    fn manager_error() {
        #[derive(PartialEq, Debug)]
        struct LooserManagerError;
        struct LooserManager;

        impl SetManager for LooserManager {
            type S = Vec<u64>;
            type E = LooserManagerError;

            fn make_set(&mut self, _size_hint: usize) -> Result<Self::S, Self::E> {
                Err(LooserManagerError)
            }

            fn reserve(&mut self, _set: &mut Self::S, _additional: usize) -> Result<(), Self::E> {
                Err(LooserManagerError)
            }
        }

        let mut sets_unioner = SetsUnion::new(LooserManager);
        assert_eq!(sets_unioner.reduce(vec![1, 2], vec![3, 4, 5]), Err(Error::SetManager(LooserManagerError)));
    }
}
