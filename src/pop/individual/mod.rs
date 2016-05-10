
pub mod chromosome;
pub mod manager;
use self::chromosome::Chromosome;

pub trait Individual: Sync + Send {
    type C: Chromosome;
    type E: Sync + Send;

    fn get_chromosome(&self) -> &Self::C;
}
