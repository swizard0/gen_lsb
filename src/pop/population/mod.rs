
pub mod manager;
pub mod vec;

use super::individual::Individual;

pub trait Population {
    type I: Individual;
    type E;

    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Result<&Self::I, Self::E>;
    fn add(&mut self, indiv: Self::I) -> Result<(), Self::E>;
    fn del(&mut self, index: usize) -> Result<Self::I, Self::E>;
}

