use std::{
    cmp::max,
    error::Error,
    path::Path,
    time::{Duration, Instant},
};

use uuid::Uuid;

use crate::vdb12::{Application, Cost, InstanceType};

const MINIMAL_RUNTIME: Duration = Duration::from_secs(60);

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

    /// Determine the minimum possible cost for scheduling the `application`.
    /// Returns `None` if deadline can not be met.
    pub fn cost(&self, application: &Application, now: Instant) -> Option<Cost> {
        let mut total = 0.0;

        for task in application.tasks.iter() {
            let cheapest_instance_type = self
                .instance_types
                .iter()
                .filter(|&instance_type| {
                    instance_type.characteristics >= task.requirements
                        && application.deadline() >= now + task.runtime
                })
                .min_by(|&left, &right| left.price.partial_cmp(&right.price).unwrap());

            let runtime = max(
                MINIMAL_RUNTIME,
                task.runtime, // FW(TmLev): consider characteristics.
            );

            total += match cheapest_instance_type {
                None => return None,
                Some(instance_type) => instance_type.price * runtime.as_secs_f64(),
            };
        }

        Some(total)
    }
}
