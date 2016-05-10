
pub mod chromosome;
use self::chromosome::Chromosome;

pub trait Individual {
    type C: Chromosome;
    type E;

    fn get_chromosome(&self) -> &Self::C;
}
