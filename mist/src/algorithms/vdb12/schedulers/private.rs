use std::error::Error;
use std::path::Path;

use crate::vdb12::{Application, InstanceType};

/// Utilisation of the private cloud resources as percentage [0, 100]%.
pub type Utilisation = f64;

/// Private cloud scheduler. Manages available resources of on-premises infrastructure.
pub struct PrivateScheduler {
    /// Restrictions of the private cloud.
    available_cpu: u32,
    available_mem: u32,
    /// Available instance types in the private cloud.
    instance_types: Vec<InstanceType>,
}

impl PrivateScheduler {
    pub fn new(instance_types: Vec<InstanceType>) -> Self {
        Self {
            // TODO(TmLev): customize.
            available_cpu: 60,
            available_mem: 64000,
            instance_types,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let raw = std::fs::read_to_string(path)?;
        let instance_types = serde_json::from_str(&raw)?;
        Ok(Self {
            available_cpu: 60,
            available_mem: 64000,
            instance_types,
        })
    }

    pub fn try_schedule(&mut self, application: Application) -> bool {
        let instance_types = self.instance_types_for_application(&application);

        let cpu: u32 = instance_types
            .iter()
            .map(|instance_type| instance_type.characteristics.cpu)
            .sum();
        let mem: u32 = instance_types
            .iter()
            .map(|instance_type| instance_type.characteristics.mem)
            .sum();

        if cpu > self.available_cpu || mem > self.available_mem {
            return false;
        }

        self.available_cpu -= cpu;
        self.available_mem -= mem;

        true
    }

    fn instance_types_for_application(&mut self, application: &Application) -> Vec<InstanceType> {
        vec![]
    }
}
