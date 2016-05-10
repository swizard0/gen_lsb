
use super::Population;
use super::super::individual::manager::IndividualManager;

pub trait PopulationManager {
    type P: Population;
    type IM: IndividualManager;
    type E;

    fn make_individual_manager(&self) -> Result<Self::IM, Self::E>;
    fn init(&self, &mut Self::IM) -> Result<Self::P, Self::E>;
}
