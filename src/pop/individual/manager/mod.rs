
use super::Individual;

pub trait IndividualManager {
    type I: Individual;
    type FI;
    type E: Sync + Send;

    fn generate(&mut self) -> Result<Self::I, Self::E>;
    fn fitness(&mut self, individual: &Self::I) -> Result<Self::FI, Self::E>;
}
