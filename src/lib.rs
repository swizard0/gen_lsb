use std::io;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

pub mod pop;

use pop::population::Population;
use pop::population::manager::PopulationManager;
use pop::individual::Individual;
use pop::individual::manager::IndividualManager;
use pop::individual::chromosome::Chromosome;

enum Command {
    Stop,
}

enum Report {
    StopAck,
}

struct Slave {
    thread: Option<thread::JoinHandle<()>>,
    tx: Sender<Command>,
    rx: Receiver<Report>,
}

impl Slave {
    fn spawn(slave_id: usize) -> Result<Slave, io::Error> {
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

impl Drop for Slave {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            self.tx.send(Command::Stop).unwrap();
            loop {
                match self.rx.recv().unwrap() {
                    Report::StopAck => break,
                    // _ => continue,
                }
            }
            thread.join().unwrap();
        }
    }
}

fn slave_loop(rx: Receiver<Command>, tx: Sender<Report>) {
    loop {
        match rx.recv().unwrap() {
            Command::Stop => {
                tx.send(Report::StopAck).unwrap();
                break;
            },
        }
    }
}

#[derive(Debug)]
pub enum RunResult<I> {
    FoundBest(I),
    PopLimitExceeded,
}

#[derive(Debug)]
pub enum Error<CE, IE, IME, PE, PME> {
    SpawnSlave(io::Error),
    Chromosome(CE),
    Individual(IE),
    IndividualManager(IME),
    Population(PE),
    PopulationManager(PME),
}

pub struct Algorithm<C, CE, I, IE, IM, IME, P, PE, PM, PME> where
    PM: PopulationManager<P = P, IM = IM, E = PME>,
    P: Population<I = I, E = PE>,
    I: Individual<C = C, E = IE>,
    IM: IndividualManager<I = I, E = IME>,
    C: Chromosome<E = CE>
{
    population_manager: Arc<PM>,
    slaves: Vec<Slave>,
}

impl<C, CE, I, IE, IM, IME, P, PE, PM, PME> Algorithm<C, CE, I, IE, IM, IME, P, PE, PM, PME> where
    PM: PopulationManager<P = P, IM = IM, E = PME>,
    P: Population<I = I, E = PE>,
    I: Individual<C = C, E = IE>,
    IM: IndividualManager<I = I, E = IME>,
    C: Chromosome<E = CE>
{
    pub fn new(population_manager: PM, slaves_count: usize) ->
        Result<Algorithm<C, CE, I, IE, IM, IME, P, PE, PM, PME>, Error<CE, IE, IME, PE, PME>>
    {
        Ok(Algorithm {
            population_manager: Arc::new(population_manager),
            slaves: try!((0 .. slaves_count).map(|i| Slave::spawn(i).map_err(|e| Error::SpawnSlave(e))).collect()),
        })
    }

    pub fn run(&mut self) -> Result<RunResult<I>, Error<CE, IE, IME, PE, PME>> {
        let mut individual_manager =
            try!(self.population_manager.make_individual_manager().map_err(|e| Error::PopulationManager(e)));
        let mut _population =
            try!(self.population_manager.init(&mut individual_manager).map_err(|e| Error::PopulationManager(e)));

        Ok(RunResult::PopLimitExceeded)
    }
}

#[cfg(test)]
mod tests {
}
