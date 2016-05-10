use std::io;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::clone::Clone;

pub mod pop;

use pop::population::Population;
use pop::population::manager::PopulationManager;
use pop::individual::Individual;
use pop::individual::manager::IndividualManager;
use pop::individual::chromosome::Chromosome;

pub trait ErrorsLayout {
    type CE: Sync + Send;
    type IE: Sync + Send;
    type IME: Sync + Send;
    type PE: Sync + Send;
    type PME: Sync + Send;
}

pub trait AlgorithmLayout: 'static {
    type EL: ErrorsLayout;
    type C: Chromosome<E = <Self::EL as ErrorsLayout>::CE>;
    type I: Individual<C = Self::C, E = <Self::EL as ErrorsLayout>::IE>;
    type IM: IndividualManager<I = Self::I, E = <Self::EL as ErrorsLayout>::IME>;
    type P: Population<I = Self::I, E = <Self::EL as ErrorsLayout>::PE>;
    type PM: PopulationManager<P = Self::P, IM = Self::IM, E = <Self::EL as ErrorsLayout>::PME>;
}

#[derive(Debug)]
pub enum Error<EL> where EL: ErrorsLayout {
    SpawnSlave(io::Error),
    Chromosome(EL::CE),
    Individual(EL::IE),
    IndividualManager(EL::IME),
    Population(EL::PE),
    PopulationManager(EL::PME),
}

enum Command<AL> where AL: AlgorithmLayout {
    PopulationInitialize(Arc<AL::PM>, Arc<AtomicUsize>),
    Stop,
}

impl<AL> Clone for Command<AL> where AL: AlgorithmLayout {
    fn clone(&self) -> Self {
        match *self {
            Command::PopulationInitialize(ref population_manager, ref sync_counter) =>
                Command::PopulationInitialize(population_manager.clone(), sync_counter.clone()),
            Command::Stop =>
                Command::Stop,
        }
    }
}

enum Report<AL> where AL: AlgorithmLayout {
    PopulationInitialized(AL::P),
    StopAck,
}

struct Slave<AL> where AL: AlgorithmLayout {
    thread: Option<thread::JoinHandle<()>>,
    tx: Sender<Command<AL>>,
    rx: Receiver<Result<Report<AL>, Error<AL::EL>>>,
}

impl<AL> Slave<AL> where AL: AlgorithmLayout {
    fn spawn(slave_id: usize) -> Result<Slave<AL>, io::Error> {
        let (master_tx, slave_rx) = channel();
        let (slave_tx, master_rx) = channel();
        let maybe_thread = thread::Builder::new()
            .name(format!("gen_lsb slave #{}", slave_id))
            .spawn(move || slave_loop(slave_rx, slave_tx));
        Ok(Slave {
            thread: Some(try!(maybe_thread)),
            tx: master_tx,
            rx: master_rx,
        })
    }
}

impl<AL> Drop for Slave<AL> where AL: AlgorithmLayout {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            self.tx.send(Command::Stop).unwrap();
            loop {
                match self.rx.recv().unwrap() {
                    Ok(Report::StopAck) => {
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

fn slave_loop<AL>(rx: Receiver<Command<AL>>, tx: Sender<Result<Report<AL>, Error<AL::EL>>>) where AL: AlgorithmLayout {
    loop {
        match rx.recv().unwrap() {
            Command::PopulationInitialize(population_manager, sync_counter) => {
                let result = slave_population_init::<AL>(population_manager, sync_counter);
                tx.send(result.map(|r| Report::PopulationInitialized(r))).unwrap();
            },
            Command::Stop => {
                tx.send(Ok(Report::StopAck)).unwrap();
                break;
            },
        }
    }
}

fn slave_population_init<AL>(population_manager: Arc<AL::PM>, sync_counter: Arc<AtomicUsize>) ->
    Result<AL::P, Error<AL::EL>> where AL: AlgorithmLayout
{
    let mut individual_manager =
        try!(population_manager.make_individual_manager().map_err(|e| Error::PopulationManager(e)));
    population_manager.init(&mut individual_manager, sync_counter).map_err(|e| Error::PopulationManager(e))
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
        let init_command =
            Command::PopulationInitialize(self.population_manager.clone(), Arc::new(AtomicUsize::new(0)));
        let _population_parts = try!(self.spread_sync(init_command));

        Ok(RunResult::PopLimitExceeded)
    }

    fn spread_sync(&self, cmd: Command<AL>) -> Result<Vec<Report<AL>>, Error<AL::EL>> {
        for slave in self.slaves.iter() {
            slave.tx.send(cmd.clone()).unwrap();
        }
        self.slaves.iter().map(|s| s.rx.recv().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
}
