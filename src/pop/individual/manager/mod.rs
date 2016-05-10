
use super::Individual;

pub trait IndividualManager {
    type I: Individual;
    type E;

    fn generate(&mut self) -> Result<Self::I, Self::E>;
}
