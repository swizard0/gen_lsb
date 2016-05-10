
pub mod pop;

use pop::builder::PopulationBuilder;
use pop::population::Population;
use pop::individual::Individual;
use pop::individual::chromosome::Chromosome;

pub enum Error<CE, IE, PE, PBE> {
    Chromosome(CE),
    Individual(IE),
    Population(PE),
    PopulationBuilder(PBE),
}

pub enum RunResult<I, E> {
    FoundBest(I),
    PopLimitExceeded,
    Error(E),
}

pub struct Algorithm<C, CE, I, IE, P, PE, PB, PBE> where
    PB: PopulationBuilder<P = P, E = PBE>,
    P: Population<I = I, E = PE>,
    I: Individual<C = C, E = IE>,
    C: Chromosome<E = CE>
{
    population_builder: PB,
}

impl<C, CE, I, IE, P, PE, PB, PBE> Algorithm<C, CE, I, IE, P, PE, PB, PBE> where
    PB: PopulationBuilder<P = P, E = PBE>,
    P: Population<I = I, E = PE>,
    I: Individual<C = C, E = IE>,
    C: Chromosome<E = CE>
{
    pub fn new(population_builder: PB) -> Algorithm<C, CE, I, IE, P, PE, PB, PBE> {
        Algorithm {
            population_builder: population_builder,
        }
    }

    pub fn run() -> RunResult<I, Error<CE, IE, PE, PBE>> {
        RunResult::PopLimitExceeded
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
