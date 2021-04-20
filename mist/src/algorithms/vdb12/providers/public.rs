use std::error::Error;
use std::path::Path;
use std::time::Instant;

use crate::vdb12::{Application, Cost, InstanceType};

pub enum ScheduleCost {
    Impossible,
    Possible(Cost),
}

#[derive(Debug, Clone)]
pub struct PublicProvider {
    instance_types: Vec<InstanceType>,
}

impl PublicProvider {
    pub fn new(instance_types: Vec<InstanceType>) -> Self {
        Self { instance_types }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let raw = std::fs::read_to_string(path)?;
        let instance_types = serde_json::from_str(&raw)?;
        Ok(Self { instance_types })
    }

    pub fn cost(&self, application: &Application, now: Instant) -> ScheduleCost {
        let mut total = 0.0;

        for task in application.tasks.iter() {
            // FIXME(TmLev):
            //   1. Check if `task` can meet deadline on `instance_type`.
            //   2. `price` should be multiplied by execution time.
            //      Requires access to simulation time.
            let cheapest_instance_type = self
                .instance_types
                .iter()
                .filter(|&instance_type| instance_type.vm >= task.minimal_vm_requirements)
                .min_by(|&left, &right| left.price.partial_cmp(&right.price).unwrap());

            total += match cheapest_instance_type {
                None => return ScheduleCost::Impossible,
                Some(instance_type) => instance_type.price,
            };
        }

        ScheduleCost::Possible(total)
    }

    pub fn schedule(&mut self, application: Application) {}
}
