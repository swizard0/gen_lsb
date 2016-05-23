use std::marker::PhantomData;
use par_exec::{Executor, ExecutorJobError, JobExecuteError};

use super::PopulationFit;
use super::super::individual::{Individual, IndividualManager};
use super::super::super::set::{Set, SetManager};
use super::super::super::set::union;

pub trait RetrieveFitsManager {
    type FitsM;

    fn retrieve(&mut self) -> &mut Self::FitsM;
}

pub trait RetrieveIndividualManager {
    type IM;

    fn retrieve(&mut self) -> &mut Self::IM;
}

pub trait Policy {
    type LocalContext: RetrieveFitsManager<FitsM = Self::FitsM> + RetrieveIndividualManager<IM = Self::IndivM>;
    type Exec: Executor<LC = Self::LocalContext>;

    type Indiv: Individual;
    type Fit;
    type IndivME: Send + 'static;
    type IndivM: IndividualManager<I = Self::Indiv, FI = Self::Fit, E = Self::IndivME>;

    type PopE: Send + 'static;
    type Pop: Set<T = Self::Indiv, E = Self::PopE>;

    type FitsE: Send + 'static;
    type Fits: Set<T = (Self::Fit, usize), E = Self::FitsE> + Send + 'static;
    type FitsME: Send + 'static;
    type FitsM: SetManager<S = Self::Fits, E = Self::FitsME>;
}

pub struct StandardPopulationFit<P> where P: Policy {
    _marker: PhantomData<P>,
}

impl<P> StandardPopulationFit<P> where P: Policy {
    pub fn new() -> StandardPopulationFit<P> {
        StandardPopulationFit {
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum FitnessError<PE, FE, FME, IME> {
    Population(PE),
    FitsSet(FE),
    FitsSetManager(FME),
    IndividualManager(IME),
}

#[derive(Debug)]
pub enum Error<ExecE, PopE, FitsE, FitsME, IndivME> {
    NoOutputFitnessValues,
    Executor(ExecutorJobError<ExecE, JobExecuteError<FitnessError<PopE, FitsE, FitsME, IndivME>, union::Error<FitsE, FitsME>>>),
}

pub type ErrorP<P> where P: Policy = Error<<P::Exec as Executor>::E, P::PopE, P::FitsE, P::FitsME, P::IndivME>;

impl<P> PopulationFit for StandardPopulationFit<P> where P: Policy {
    type Exec = P::Exec;
    type Indiv = P::Indiv;
    type Pop = P::Pop;
    type Fit = P::Fit;
    type Fits = P::Fits;
    type Err = ErrorP<P>;

    fn fit(&self, _population: &Self::Pop, _exec: &mut Self::Exec) -> Result<Self::Fits, Self::Err> {
        Err(Error::NoOutputFitnessValues::<<Self::Exec as Executor>::E, P::PopE, P::FitsE, P::FitsME, P::IndivME>)
    }
}
