pub mod pop;

use pop::population::Population;
use pop::population::manager::PopulationManager;
use pop::individual::Individual;
use pop::individual::manager::IndividualManager;
use pop::individual::chromosome::Chromosome;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error<CE, IE, IME, PE, PME> {
    Chromosome(CE),
    Individual(IE),
    IndividualManager(IME),
    Population(PE),
    PopulationManager(PME),
}

pub enum RunResult<I> {
    FoundBest(I),
    PopLimitExceeded,
}

pub struct Algorithm<C, CE, I, IE, IM, IME, P, PE, PM, PME> where
    PM: PopulationManager<P = P, IM = IM, E = PME>,
    P: Population<I = I, E = PE>,
    I: Individual<C = C, E = IE>,
    IM: IndividualManager<I = I, E = IME>,
    C: Chromosome<E = CE>
{
    population_manager: PM,
    individual_manager: IM,
}

impl<C, CE, I, IE, IM, IME, P, PE, PM, PME> Algorithm<C, CE, I, IE, IM, IME, P, PE, PM, PME> where
    PM: PopulationManager<P = P, IM = IM, E = PME>,
    P: Population<I = I, E = PE>,
    I: Individual<C = C, E = IE>,
    IM: IndividualManager<I = I, E = IME>,
    C: Chromosome<E = CE>
{
    pub fn new(population_manager: PM, individual_manager: IM) -> Algorithm<C, CE, I, IE, IM, IME, P, PE, PM, PME> {
        Algorithm {
            population_manager: population_manager,
            individual_manager: individual_manager,
        }
    }

    pub fn run(&mut self) -> Result<RunResult<I>, Error<CE, IE, IME, PE, PME>> {
        let mut _population =
            try!(self.population_manager.init(&mut self.individual_manager).map_err(|e| Error::PopulationManager(e)));
        
        Ok(RunResult::PopLimitExceeded)
    }
}

#[cfg(test)]
mod tests {
}
