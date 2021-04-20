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
    /// Bag of tasks.
    pub tasks: Vec<Task>,
    /// Way to differentiate applications.
    uuid: Uuid,
    /// Deadline to meet. May be used as a key in the sorted data structures.
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

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn deadline(&self) -> Deadline {
        self.deadline
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl PartialEq for Application {
    fn eq(&self, other: &Application) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Application {}
