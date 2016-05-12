use std::io;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::clone::Clone;

pub mod pop;
pub mod set;

use set::Set;
use pop::population::manager::{PopulationManager, PopulationJobs};
use pop::individual::Individual;
use pop::individual::manager::IndividualManager;
use pop::individual::chromosome::Chromosome;

pub trait ErrorsLayout {
    type CE: Sync + Send;
    type IE: Sync + Send;
    type IME: Sync + Send;
    type PE: Sync + Send;
    type PME: Sync + Send;
    type PJE: Sync + Send;
}

pub trait AlgorithmLayout: 'static {
    type EL: ErrorsLayout;
    type C: Chromosome<E = <Self::EL as ErrorsLayout>::CE>;
    type I: Individual<C = Self::C, E = <Self::EL as ErrorsLayout>::IE>;
    type IM: IndividualManager<I = Self::I, E = <Self::EL as ErrorsLayout>::IME>;
    type P: Set<T = Self::I, E = <Self::EL as ErrorsLayout>::PE>;
    type PJ: PopulationJobs<I = Self::I, P = Self::P, IM = Self::IM, E = <Self::EL as ErrorsLayout>::PJE>;
    type PM: PopulationManager<PJ = Self::PJ, IM = Self::IM, E = <Self::EL as ErrorsLayout>::PME>;
}

#[derive(Debug)]
pub enum Error<EL> where EL: ErrorsLayout {
    SpawnSlave(io::Error),
    Chromosome(EL::CE),
    Individual(EL::IE),
    IndividualManager(EL::IME),
    Population(EL::PE),
    PopulationManager(EL::PME),
    PopulationJobs(EL::PJE),
    UnexpectedMasterCommand,
    UnexpectedSlaveReply,
    UnexpectedEmptyPopulation,
    Several(Vec<Error<EL>>),
}

enum Command<AL> where AL: AlgorithmLayout {
    SlaveStart(Arc<AL::PM>),
    PopulationInitialize(Arc<AtomicUsize>),
    SlaveStop,
    SlaveQuit,
}

impl<AL> Clone for Command<AL> where AL: AlgorithmLayout {
    fn clone(&self) -> Self {
        match *self {
            Command::SlaveStart(ref population_manager) =>
                Command::SlaveStart(population_manager.clone()),
            Command::PopulationInitialize(ref sync_counter) =>
                Command::PopulationInitialize(sync_counter.clone()),
            Command::SlaveStop =>
                Command::SlaveStop,
            Command::SlaveQuit =>
                Command::SlaveQuit,
        }
    }
}

enum Report<AL> where AL: AlgorithmLayout {
    SlaveStarted,
    PopulationInitialized(AL::P),
    SlaveStopped,
    SlaveQuitted,
}

impl<AL> Report<AL> where AL: AlgorithmLayout {
    fn population_results(self) -> AL::P {
        match self {
            Report::PopulationInitialized(population) => population,
            _ => unreachable!(),
        }
    }

    fn same_type(&self, other: &Report<AL>) -> bool {
        match (self, other) {
            (&Report::SlaveStarted, &Report::SlaveStarted) => true,
            (&Report::PopulationInitialized(..), &Report::PopulationInitialized(..)) => true,
            (&Report::SlaveStopped, &Report::SlaveStopped) => true,
            (&Report::SlaveQuitted, &Report::SlaveQuitted) => true,
            _ => false,
        }
    }
}

struct Slave<AL> where AL: AlgorithmLayout {
    thread: Option<thread::JoinHandle<()>>,
    tx: Sender<Command<AL>>,
    rx: Receiver<Result<Report<AL>, Error<AL::EL>>>,
    last_report: Option<Report<AL>>,
}

impl<AL> Slave<AL> where AL: AlgorithmLayout {
    fn spawn(slave_id: usize) -> Result<Slave<AL>, io::Error> {
        let (master_tx, slave_rx) = channel();
        let (slave_tx, master_rx) = channel();
        let maybe_thread = thread::Builder::new()
            .name(format!("gen_lsb slave #{}", slave_id))
            .spawn(move || slave_idle(slave_rx, slave_tx));
        Ok(Slave {
            thread: Some(try!(maybe_thread)),
            tx: master_tx,
            rx: master_rx,
            last_report: None,
        })
    }
}

impl<AL> Drop for Slave<AL> where AL: AlgorithmLayout {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            self.tx.send(Command::SlaveQuit).unwrap();
            loop {
                match self.rx.recv().unwrap() {
                    Ok(Report::SlaveQuitted) => {
                        thread.join().unwrap();
                        break;
                    },
                    Ok(_) =>
                        continue,
                    Err(_err) =>
                        unreachable!(),
                }
            }
        }
    }
}

fn slave_idle<AL>(rx: Receiver<Command<AL>>, tx: Sender<Result<Report<AL>, Error<AL::EL>>>) where AL: AlgorithmLayout {
    loop {
        match rx.recv().unwrap() {
            Command::SlaveStart(population_manager) =>
                match population_manager.make_individual_manager() {
                    Ok(individual_manager) =>
                        slave_loop(&*population_manager, individual_manager, &rx, &tx),
                    Err(err) =>
                        tx.send(Err(Error::PopulationManager(err))).unwrap(),
                },
            Command::SlaveQuit => {
                tx.send(Ok(Report::SlaveQuitted)).unwrap();
                break;
            },
            _ =>
                tx.send(Err(Error::UnexpectedMasterCommand)).unwrap(),
        }
    }
}

struct SyncIter(Arc<AtomicUsize>);

impl Iterator for SyncIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        Some(self.0.fetch_add(1, Ordering::Relaxed))
    }
}

fn slave_loop<AL>(population_manager: &AL::PM,
                  mut individual_manager: AL::IM,
                  rx: &Receiver<Command<AL>>,
                  tx: &Sender<Result<Report<AL>, Error<AL::EL>>>) where AL: AlgorithmLayout
{
    let population_jobs = population_manager.jobs();
    loop {
        match rx.recv().unwrap() {
            Command::PopulationInitialize(sync_counter) =>
                tx.send(match population_jobs.init(&mut individual_manager, SyncIter(sync_counter)) {
                    Ok(population) => Ok(Report::PopulationInitialized(population)),
                    Err(err) => Err(Error::PopulationJobs(err)),
                }).unwrap(),
            Command::SlaveStop => {
                tx.send(Ok(Report::SlaveStopped)).unwrap();
                break;
            },
            Command::SlaveStart(..) | Command::SlaveQuit =>
                tx.send(Err(Error::UnexpectedMasterCommand)).unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum RunResult<I> {
    FoundBest(I),
    PopLimitExceeded,
}

pub struct Algorithm<AL> where AL: AlgorithmLayout {
    population_manager: Arc<AL::PM>,
    slaves: Vec<Slave<AL>>,
}

impl<AL> Algorithm<AL> where AL: AlgorithmLayout {
    pub fn new(population_manager: AL::PM, slaves_count: usize) -> Result<Algorithm<AL>, Error<AL::EL>> {
        Ok(Algorithm {
            population_manager: Arc::new(population_manager),
            slaves: try!((0 .. slaves_count).map(|i| Slave::spawn(i).map_err(|e| Error::SpawnSlave(e))).collect()),
        })
    }

    pub fn run(&mut self) -> Result<RunResult<AL::I>, Error<AL::EL>> {
        let start_cmd = Command::SlaveStart(self.population_manager.clone());
        try!(self.spread_sync_expect(start_cmd, Report::SlaveStarted));
        match (self.run_with_guard(), self.spread_sync_expect(Command::SlaveStop, Report::SlaveStopped)) {
            (Ok(r), Ok(())) => Ok(r),
            (Ok(..), Err(err)) => Err(err),
            (Err(err), Ok(())) => Err(err),
            (Err(ea), Err(eb)) => Err(Error::Several(vec![ea, eb])),
        }
    }

    fn run_with_guard(&mut self) -> Result<RunResult<AL::I>, Error<AL::EL>> {
        let _population = self.population_initialize();

        Ok(RunResult::PopLimitExceeded)
    }

    pub fn population_initialize(&mut self) -> Result<AL::P, Error<AL::EL>> {
        try!(self.spread_sync(Command::PopulationInitialize(Arc::new(AtomicUsize::new(0)))));
        let maybe_population =
            Set::merge_many(self.slaves.iter_mut().flat_map(|s| s.last_report.take()).map(|r| r.population_results()));
        try!(maybe_population.map_err(|e| Error::Population(e))).ok_or(Error::UnexpectedEmptyPopulation)
    }

    fn spread_sync(&mut self, cmd: Command<AL>) -> Result<(), Error<AL::EL>> {
        for slave in self.slaves.iter() {
            slave.tx.send(cmd.clone()).unwrap();
        }
        let mut errors = Vec::new();
        for slave in self.slaves.iter_mut() {
            match slave.rx.recv().unwrap() {
                Ok(r) =>
                    slave.last_report = Some(r),
                Err(e) =>
                    errors.push(e),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.pop().unwrap())
        } else {
            Err(Error::Several(errors))
        }
    }

    fn spread_sync_expect(&mut self, cmd: Command<AL>, rep: Report<AL>) -> Result<(), Error<AL::EL>> {
        try!(self.spread_sync(cmd));
        for slave_reply in self.slaves.iter_mut().map(|s| s.last_report.take()) {
            if let Some(true) = slave_reply.map(|r| r.same_type(&rep)) {
                continue;
            } else {
                return Err(Error::UnexpectedSlaveReply);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
}
