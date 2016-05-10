
pub trait Chromosome: Sync + Send {
    type E: Sync + Send;
}
