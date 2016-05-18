use super::set::Set;

pub mod individual;
use self::individual::Individual;

pub trait PopulationInit {
    type Indiv: Individual;
    type Pop: Set<T = Self::Indiv>;
    type Err;

    fn init(amount: usize) -> Result<Self::Pop, Self::Err>;
}
