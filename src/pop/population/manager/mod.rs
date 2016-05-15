use super::super::super::set::Set;
use super::super::individual::Individual;
use super::super::individual::manager::IndividualManager;

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
    type FI;
    type IM: IndividualManager<I = Self::I, FI = Self::FI>;
    type FS: Set<T = (usize, Self::FI)>;
    type E: Sync + Send;

    fn init<IT>(&self, individual_manager: &mut Self::IM, indices: IT) -> Result<Self::P, Self::E> where IT: Iterator<Item = usize>;
    fn fitness<'a, IT>(&self, individual_manager: &mut Self::IM, individuals: IT) -> Result<Self::FS, Self::E>
        where IT: Iterator<Item = (usize, &'a Self::I)>, Self::I: 'a;
}

