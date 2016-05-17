use par_exec::Reducer;

// pub mod vec;

pub trait Set {
    type T;
    type E;

    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Result<&Self::T, Self::E>;
    fn add(&mut self, item: Self::T) -> Result<(), Self::E>;
}

pub trait SetManager {
    type S;
    type E;

    fn make_set(&mut self, size_hint: usize) -> Result<Self::S, Self::E>;
}

pub enum SetsMergerError<ES, ESM> {
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
    S: Set<T = T, E = ES>,
    SM: SetManager<S = S, E = ESM>
{
    type R = S;
    type E = SetsMergerError<ES, ESM>;

    fn len(&self, item: &Self::R) -> Option<usize> {
        Some(item.size())
    }
    
    fn reduce(&mut self, item_a: Self::R, item_b: Self::R) -> Result<Self::R, Self::E> {
        let (limit_i, limit_j) = (item_a.size(), item_b.size());
        let mut target =
            try!(self.set_manager.make_set(limit_i + limit_j).map_err(|e| SetsMergerError::SetManager(e)));
        let (mut i, mut j) = (0, 0);

        Ok(item_a)
        // while (i < limit_i) && (j < limit_j) {
            
        // }
    }
}

