
pub mod chromosome;
pub mod manager;
use self::chromosome::Chromosome;

pub trait Individual {
    type C: Chromosome;
    type E;

    fn get_chromosome(&self) -> &Self::C;
}
