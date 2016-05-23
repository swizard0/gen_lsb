use super::{Set, SetManager};

#[derive(PartialEq, Debug)]
pub enum Error<ES, ESM> {
    Set(ES),
    SetManager(ESM),
}

pub fn union<S, SE, SM, SME>(set_manager: &mut SM, mut item_a: S, item_b: S) -> Result<S, Error<SE, SME>> where
    S: Set<E = SE>,
    SM: SetManager<S = S, E = SME>
{
    try!(set_manager.reserve(&mut item_a, item_b.size()).map_err(Error::SetManager));
    for maybe_value in item_b.into_iter() {
        let value = try!(maybe_value.map_err(Error::Set));
        try!(item_a.add(value).map_err(Error::Set));
    }
    Ok(item_a)
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::collections::HashSet;
    use self::rand::Rng;
    use super::{Error, union};
    use super::super::SetManager;
    use super::super::vec::Manager;

    #[test]
    fn union_sorted_vecs() {
        let mut rng = rand::thread_rng();
        let vec_a: Vec<u64> = (0 .. 1024).map(|_| rng.gen()).collect();
        let vec_b: Vec<u64> = (0 .. 768).map(|_| rng.gen()).collect();
        let vec_a_clone = vec_a.clone();
        let vec_b_clone = vec_b.clone();

        let mut set_manager = Manager::new();

        let vec_c = union(&mut set_manager, vec_a, vec_b).unwrap();
        assert_eq!(vec_c.len(), 1024 + 768);

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

            fn make_set(&mut self, _size_hint: Option<usize>) -> Result<Self::S, Self::E> {
                Err(LooserManagerError)
            }

            fn reserve(&mut self, _set: &mut Self::S, _additional: usize) -> Result<(), Self::E> {
                Err(LooserManagerError)
            }
        }

        let mut set_manager = LooserManager;
        assert_eq!(union(&mut set_manager, vec![1, 2], vec![3, 4, 5]), Err(Error::SetManager(LooserManagerError)));
    }
}
