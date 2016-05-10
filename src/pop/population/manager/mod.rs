
use super::Population;
use super::super::individual::manager::IndividualManager;

pub trait PopulationManager {
    type P: Population;
    type IM: IndividualManager;
    type E;

    fn init(&mut self, &mut Self::IM) -> Result<Self::P, Self::E>;
}
