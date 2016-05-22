

pub trait Individual {
    type E;
}

pub trait IndividualManager {
    type I: Individual;
    type E;

    fn generate(&mut self) -> Result<Self::I, Self::E>;
}

pub trait IndividualManagerMut {
    type IM: IndividualManager;

    fn individual_manager_mut(&mut self) -> &mut Self::IM;
}
