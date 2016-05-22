use super::{Set, SetManager, SetManagerMut};

#[derive(PartialEq, Debug)]
pub enum Error<ES, ESM> {
    Set(ES),
    SetManager(ESM),
}

pub fn merge<LC, T, S, SE, SM, SME>(local_context: &mut LC, item_a: S, item_b: S) -> Result<S, Error<SE, SME>> where
    LC: SetManagerMut<SM = SM>,
    T: PartialOrd,
    S: Set<T = T, E = SE>,
    SM: SetManager<S = S, E = SME>
{
    let (limit_a, limit_b) = (item_a.size(), item_b.size());
    let set_manager = local_context.set_manager_mut();
    let mut target =
        try!(set_manager.make_set(Some(limit_a + limit_b)).map_err(|e| Error::SetManager(e)));
    let (mut iter_a, mut iter_b) = (item_a.into_iter(), item_b.into_iter());
    let (mut curr_a, mut curr_b) = (iter_a.next(), iter_b.next());
    loop {
        let (value, next_a, next_b) = match (curr_a, curr_b) {
            (None, None) =>
                return Ok(target),
            (Some(Err(e)), _) | (_, Some(Err(e))) =>
                return Err(Error::Set(e)),
            (None, Some(Ok(value_b))) =>
                (value_b, None, iter_b.next()),
            (Some(Ok(value_a)), None) =>
                (value_a, iter_a.next(), None),
            (Some(Ok(value_a)), Some(Ok(value_b))) => if value_a < value_b {
                (value_a, iter_a.next(), Some(Ok(value_b)))
            } else {
                (value_b, Some(Ok(value_a)), iter_b.next())
            },
        };

        curr_a = next_a;
        curr_b = next_b;
        try!(target.add(value).map_err(|e| Error::Set(e)));
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::Rng;
    use super::{Error, merge};
    use super::super::{SetManager, SetManagerMut};
    use super::super::vec::Manager;

    #[test]
    fn merge_sorted_vecs() {
        let mut rng = rand::thread_rng();
        let mut vec_a: Vec<u64> = (0 .. 1024).map(|_| rng.gen()).collect();
        let mut vec_b: Vec<u64> = (0 .. 768).map(|_| rng.gen()).collect();
        vec_a.sort();
        vec_b.sort();

        struct LocalContext<T>(Manager<T>);
        impl<T> SetManagerMut for LocalContext<T> {
            type SM = Manager<T>;

            fn set_manager_mut(&mut self) -> &mut Manager<T> {
                &mut self.0
            }
        }

        let mut local_context = LocalContext(Manager::new());

        let vec_c = merge(&mut local_context, vec_a, vec_b).unwrap();
        assert_eq!(vec_c.len(), 1024 + 768);

        for i in 1 .. 1024 + 768 {
            assert!(vec_c[i - 1] <= vec_c[i]);
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
        assert_eq!(merge(&mut local_context, vec![1, 2], vec![3, 4, 5]), Err(Error::SetManager(LooserManagerError)));
    }
}
