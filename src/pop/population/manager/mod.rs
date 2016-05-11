use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use super::Population;
use super::super::individual::manager::IndividualManager;

pub mod simple;

pub trait PopulationManager: Sync + Send {
    type PJ: PopulationJobs;
    type IM: IndividualManager;
    type E: Sync + Send;

    fn make_individual_manager(&self) -> Result<Self::IM, Self::E>;
    fn jobs(&self) -> &Self::PJ;
}

pub trait PopulationJobs {
    type P: Population;
    type IM: IndividualManager;
    type E: Sync + Send;

    fn init(&self, individual_manager: &mut Self::IM, sync_counter: Arc<AtomicUsize>) -> Result<Self::P, Self::E>;
}

