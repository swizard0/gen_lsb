use super::{Set, SetManager};

#[derive(PartialEq, Debug)]
pub enum Error<ES, ESM> {
    Set(ES),
    SetManager(ESM),
}

pub fn merge<T, S, SE, SM, SME, F>(set_manager: &mut SM, set_a: S, set_b: S, mut pred: F) -> Result<S, Error<SE, SME>> where
    S: Set<T = T, E = SE>,
    SM: SetManager<S = S, E = SME>,
    F: FnMut(&T, &T) -> bool
{
    let (limit_a, limit_b) = (set_a.size(), set_b.size());
    let mut target =
        try!(set_manager.make_set(Some(limit_a + limit_b)).map_err(Error::SetManager));
    let (mut iter_a, mut iter_b) = (set_a.into_iter(), set_b.into_iter());
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
            (Some(Ok(value_a)), Some(Ok(value_b))) => if pred(&value_a, &value_b) {
                (value_a, iter_a.next(), Some(Ok(value_b)))
            } else {
                (value_b, Some(Ok(value_a)), iter_b.next())
            },
        };

        curr_a = next_a;
        curr_b = next_b;
        try!(target.add(value).map_err(Error::Set));
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::Rng;
    use super::{Error, merge};
    use super::super::SetManager;
    use super::super::vec::Manager;

    #[test]
    fn merge_sorted_vecs() {
        let mut rng = rand::thread_rng();
        let mut vec_a: Vec<u64> = (0 .. 1024).map(|_| rng.gen()).collect();
        let mut vec_b: Vec<u64> = (0 .. 768).map(|_| rng.gen()).collect();
        vec_a.sort();
        vec_b.sort();

        let mut set_manager = Manager::new();
        let vec_c = merge(&mut set_manager, vec_a, vec_b, |a, b| a < b).unwrap();
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

        let mut set_manager = LooserManager;
        assert_eq!(merge(&mut set_manager, vec![1, 2], vec![3, 4, 5], |a, b| a < b), Err(Error::SetManager(LooserManagerError)));
    }
}
