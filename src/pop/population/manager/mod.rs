use super::super::super::set::Set;
use super::super::individual::Individual;
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
    type I: Individual;
    type P: Set<T = Self::I>;
    type IM: IndividualManager<I = Self::I>;
    type E: Sync + Send;

    fn init<IT>(&self, individual_manager: &mut Self::IM, indices: IT) -> Result<Self::P, Self::E> where IT: Iterator<Item = usize>;
}

