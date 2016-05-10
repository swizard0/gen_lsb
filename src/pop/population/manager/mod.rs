use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use super::Population;
use super::super::individual::manager::IndividualManager;

pub trait PopulationManager: Sync + Send {
    type P: Population;
    type IM: IndividualManager;
    type E: Sync + Send;

    fn make_individual_manager(&self) -> Result<Self::IM, Self::E>;
    fn init(&self, &mut Self::IM, sync_counter: Arc<AtomicUsize>) -> Result<Self::P, Self::E>;
}
