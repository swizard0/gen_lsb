
pub mod vec;

pub trait Set: Sized + Sync + Send {
    type T;
    type E: Sync + Send;

    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Result<&Self::T, Self::E>;
    fn add(&mut self, indiv: Self::T) -> Result<(), Self::E>;
    fn del(&mut self, index: usize) -> Result<Self::T, Self::E>;
    fn merge(self, other: Self) -> Result<Self, Self::E>;

    fn merge_many<IT>(sets: IT) -> Result<Option<Self>, Self::E> where IT: Iterator<Item = Self> {
        let mut result: Option<Self> = None;
        for set in sets {
            result = Some(if let Some(current_set) = result.take() {
                try!(current_set.merge(set))
            } else {
                set
            });
        }
        Ok(result)
    }
}

pub trait SetEmpty: Sized + Sync + Send {
    type E: Sync + Send;

    fn make_empty() -> Result<Self, Self::E>;
}
