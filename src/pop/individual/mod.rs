

pub trait Individual {
    type E;
}

pub trait IndividualManager {
    type I: Individual;
    type FI;
    type E;

    fn generate(&mut self, index: usize) -> Result<Self::I, Self::E>;
    fn fitness(&mut self, indiv: &Self::I) -> Result<Self::FI, Self::E>;
}

pub trait IndividualManagerMut {
    type IM: IndividualManager;

    fn individual_manager_mut(&mut self) -> &mut Self::IM;
}
