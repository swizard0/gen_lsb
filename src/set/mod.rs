pub mod vec;
pub mod merge;
pub mod union;

pub trait Set {
    type T;
    type E;
    type I: Iterator<Item = Result<Self::T, Self::E>>;

    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Result<&Self::T, Self::E>;
    fn add(&mut self, item: Self::T) -> Result<(), Self::E>;
    fn into_iter(self) -> Self::I;
}

pub trait SetManager {
    type S: Set;
    type E;

    fn make_set(&mut self, size_hint: usize) -> Result<Self::S, Self::E>;
    fn reserve(&mut self, set: &mut Self::S, additional: usize) -> Result<(), Self::E>;
}
