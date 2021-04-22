use std::error::Error;
use std::path::Path;

use crate::vdb12::{Application, InstanceType};

pub struct PrivateScheduler {
    /// Restrictions of the private cloud.
    max_cpu: u32,
    max_mem: u32,
    /// Available instance types in the private cloud.
    instance_types: Vec<InstanceType>,
}

impl PrivateScheduler {
    pub fn new(instance_types: Vec<InstanceType>) -> Self {
        Self {
            // TODO(TmLev): customize.
            max_cpu: 60,
            max_mem: 64000,
            instance_types,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let raw = std::fs::read_to_string(path)?;
        let instance_types = serde_json::from_str(&raw)?;
        Ok(Self {
            max_cpu: 60,
            max_mem: 64000,
            instance_types,
        })
    }

    pub fn try_schedule(&mut self, application: Application) {}
}
