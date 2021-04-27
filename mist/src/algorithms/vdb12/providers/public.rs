use std::{
    cmp::max,
    error::Error,
    path::Path,
    time::{Duration, Instant},
};

use uuid::Uuid;

use crate::vdb12::{Application, Cost, InstanceType};

pub enum ScheduleCost {
    Impossible,
    Possible(Cost),
}

#[derive(Debug, Clone)]
pub struct PublicProvider {
    uuid: Uuid,
    instance_types: Vec<InstanceType>,
}

impl PublicProvider {
    pub fn new(instance_types: Vec<InstanceType>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            instance_types,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let raw = std::fs::read_to_string(path)?;
        let instance_types = serde_json::from_str(&raw)?;
        Ok(Self {
            uuid: Uuid::new_v4(),
            instance_types,
        })
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn cost(&self, application: &Application, now: Instant) -> ScheduleCost {
        let mut total = 0.0;

        for task in application.tasks.iter() {
            // FIXME(TmLev): Check if `task` can meet deadline on `instance_type`.
            let cheapest_instance_type = self
                .instance_types
                .iter()
                .filter(|&instance_type| instance_type.vm >= task.minimal_vm_requirements)
                .min_by(|&left, &right| left.price.partial_cmp(&right.price).unwrap());

            let runtime = max(Duration::from_secs(60), task.runtime);

            total += match cheapest_instance_type {
                None => return ScheduleCost::Impossible,
                Some(instance_type) => instance_type.price * runtime,
            };
        }

        ScheduleCost::Possible(total)
    }
}
