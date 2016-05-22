use super::{Set, SetManager, SetManagerMut};

#[derive(PartialEq, Debug)]
pub enum Error<ES, ESM> {
    Set(ES),
    SetManager(ESM),
}

pub fn union<LC, S, SE, SM, SME>(local_context: &mut LC, mut item_a: S, item_b: S) -> Result<S, Error<SE, SME>> where
    LC: SetManagerMut<SM = SM>,
    S: Set<E = SE>,
    SM: SetManager<S = S, E = SME>
{
    let set_manager = local_context.set_manager_mut();
    try!(set_manager.reserve(&mut item_a, item_b.size()).map_err(|e| Error::SetManager(e)));
    for maybe_value in item_b.into_iter() {
        let value = try!(maybe_value.map_err(|e| Error::Set(e)));
        try!(item_a.add(value).map_err(|e| Error::Set(e)));
    }
    Ok(item_a)
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::collections::HashSet;
    use self::rand::Rng;
    use super::{Error, union};
    use super::super::{SetManager, SetManagerMut};
    use super::super::vec::Manager;

    #[test]
    fn union_sorted_vecs() {
        let mut rng = rand::thread_rng();
        let vec_a: Vec<u64> = (0 .. 1024).map(|_| rng.gen()).collect();
        let vec_b: Vec<u64> = (0 .. 768).map(|_| rng.gen()).collect();
        let vec_a_clone = vec_a.clone();
        let vec_b_clone = vec_b.clone();

        struct LocalContext<T>(Manager<T>);
        impl<T> SetManagerMut for LocalContext<T> {
            type SM = Manager<T>;

            fn set_manager_mut(&mut self) -> &mut Manager<T> {
                &mut self.0
            }
        }

        let mut local_context = LocalContext(Manager::new());

        let vec_c = union(&mut local_context, vec_a, vec_b).unwrap();
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

        struct LocalContext(LooserManager);
        impl SetManagerMut for LocalContext {
            type SM = LooserManager;

            fn set_manager_mut(&mut self) -> &mut LooserManager {
                &mut self.0
            }
        }

        let mut local_context = LocalContext(LooserManager);
        assert_eq!(union(&mut local_context, vec![1, 2], vec![3, 4, 5]), Err(Error::SetManager(LooserManagerError)));
    }
}
