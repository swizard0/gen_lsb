
pub mod pop;

pub enum Error {
    Population(pop::Error),
}

pub struct Algorithm {

}

impl Algorithm {
    pub fn new() -> Algorithm {
        Algorithm {
        }
    }

    pub fn run() -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
