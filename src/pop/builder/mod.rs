
use super::population::Population;

pub trait PopulationBuilder {
    type P: Population;
    type E;

    fn init_new(&mut self) -> Result<Self::P, Self::E>;
}
