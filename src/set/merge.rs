use par_exec::Reducer;
use super::{Set, SetManager};

#[derive(Debug)]
pub enum Error<ES, ESM> {
    Set(ES),
    SetManager(ESM),
}

pub struct SetsMerger<SM> {
    set_manager: SM,
}

impl<SM> SetsMerger<SM> {
    pub fn new(set_manager: SM) -> SetsMerger<SM> {
        SetsMerger {
            set_manager: set_manager,
        }
    }
}

impl<S, T, ES, SM, ESM> Reducer for SetsMerger<SM> where
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
    use super::{Error, SetsMerger};
    use su
    
}
