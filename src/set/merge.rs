use par_exec::Reducer;
use super::{Set, SetManager};

#[derive(PartialEq, Debug)]
pub enum Error<ES, ESM> {
    Set(ES),
    SetManager(ESM),
}

pub struct SetsMerge<SM> {
    set_manager: SM,
}

impl<SM> SetsMerge<SM> {
    pub fn new(set_manager: SM) -> SetsMerge<SM> {
        SetsMerge {
            set_manager: set_manager,
        }
    }
}

impl<S, T, ES, SM, ESM> Reducer for SetsMerge<SM> where
    T: PartialOrd,
    S: Set<T = T, E = ES>,
    SM: SetManager<S = S, E = ESM>
{
    type R = S;
    type E = Error<ES, ESM>;

    fn len(&self, item: &Self::R) -> Option<usize> {
        Some(item.size())
    }

    fn reduce(&mut self, item_a: Self::R, item_b: Self::R) -> Result<Self::R, Self::E> {
        let (limit_a, limit_b) = (item_a.size(), item_b.size());
        let mut target =
            try!(self.set_manager.make_set(limit_a + limit_b).map_err(|e| Error::SetManager(e)));
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
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use par_exec::Reducer;
    use self::rand::Rng;
    use super::{Error, SetsMerge};
    use super::super::SetManager;
    use super::super::vec::Manager;

    #[test]
    fn merge_sorted_vecs() {
        let mut rng = rand::thread_rng();
        let mut vec_a: Vec<u64> = (0 .. 1024).map(|_| rng.gen()).collect();
        let mut vec_b: Vec<u64> = (0 .. 768).map(|_| rng.gen()).collect();
        vec_a.sort();
        vec_b.sort();

        let mut sets_merger = SetsMerge::new(Manager::new());

        assert_eq!(sets_merger.len(&vec_a), Some(1024));
        assert_eq!(sets_merger.len(&vec_b), Some(768));

        let vec_c = sets_merger.reduce(vec_a, vec_b).unwrap();
        assert_eq!(sets_merger.len(&vec_c), Some(1024 + 768));

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

            fn make_set(&mut self, _size_hint: usize) -> Result<Self::S, Self::E> {
                Err(LooserManagerError)
            }
        }

        let mut sets_merger = SetsMerge::new(LooserManager);
        assert_eq!(sets_merger.reduce(vec![1, 2], vec![3, 4, 5]), Err(Error::SetManager(LooserManagerError)));
    }
}
