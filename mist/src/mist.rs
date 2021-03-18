use log;

use crate::Algorithm;

pub struct Mist {}

impl Mist {
    pub fn new(algorithm: Algorithm) -> Self {
        match algorithm {
            Algorithm::Vdb12 => Self {},
        }
    }

    pub fn run(&mut self) {
        log::info!("Starting simulation");
    }
}
