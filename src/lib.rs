extern crate par_exec;

// pub mod pop;
pub mod set;

use set::Set;




// use pop::population::manager::{PopulationManager, PopulationJobs};
// use pop::individual::Individual;
// use pop::individual::manager::IndividualManager;
// use pop::individual::chromosome::Chromosome;

// pub trait ErrorsLayout {
//     type CE: Sync + Send;
//     type IE: Sync + Send;
//     type IME: Sync + Send;
//     type PE: Sync + Send;
//     type PME: Sync + Send;
//     type PJE: Sync + Send;
// }

// pub trait AlgorithmLayout: 'static {
//     type EL: ErrorsLayout;
//     type C: Chromosome<E = <Self::EL as ErrorsLayout>::CE>;
//     type I: Individual<C = Self::C, E = <Self::EL as ErrorsLayout>::IE>;
//     type FI;
//     type IM: IndividualManager<I = Self::I, FI = Self::FI, E = <Self::EL as ErrorsLayout>::IME>;
//     type P: Set<T = Self::I, E = <Self::EL as ErrorsLayout>::PE>;
//     type PJ: PopulationJobs<I = Self::I, P = Self::P, FI = Self::FI, IM = Self::IM, E = <Self::EL as ErrorsLayout>::PJE>;
//     type PM: PopulationManager<PJ = Self::PJ, IM = Self::IM, E = <Self::EL as ErrorsLayout>::PME>;
// }


#[cfg(test)]
mod tests {
}
