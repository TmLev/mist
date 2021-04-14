use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::vdb12::Vm;

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Task {
    pub minimal_vm_requirements: Vm,
}

pub type Deadline = DateTime<Utc>;

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Application {
    // Bag of tasks.
    pub tasks: Vec<Task>,
    // Way to differentiate applications.
    uuid: Uuid,
    // Deadline to meet.
    // Private because it may be used as key in sorted data structures.
    deadline: Deadline,
}

impl Application {
    pub fn new(tasks: Vec<Task>, deadline: Deadline) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            tasks,
            deadline,
        }
    }

    // Builder pattern.
    pub fn with_uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = uuid;
        self
    }

    pub fn deadline(&self) -> Deadline {
        self.deadline
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
