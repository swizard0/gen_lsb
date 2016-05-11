
pub mod manager;
pub mod vec;

use super::individual::Individual;

pub trait Population: Sized + Sync + Send {
    type I: Individual;
    type E: Sync + Send;

    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Result<&Self::I, Self::E>;
    fn add(&mut self, indiv: Self::I) -> Result<(), Self::E>;
    fn del(&mut self, index: usize) -> Result<Self::I, Self::E>;
    fn merge(self, other: Self) -> Result<Self, Self::E>;

    fn merge_many<IT>(populations: IT) -> Result<Option<Self>, Self::E> where IT: Iterator<Item = Self> {
        let mut result: Option<Self> = None;
        for pop in populations {
            result = Some(if let Some(current_pop) = result.take() {
                try!(current_pop.merge(pop))
            } else {
                pop
            });
        }
        Ok(result)
    }
}

pub trait PopulationEmpty: Sized + Sync + Send {
    type E: Sync + Send;

    fn make_empty() -> Result<Self, Self::E>;
}

