use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::vdb12::Vm;

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Task {
    pub minimal_vm_requirements: Vm,
    pub deadline: DateTime<Utc>,
}

impl Task {
    pub fn new(minimal_vm_requirements: Vm, deadline: DateTime<Utc>) -> Self {
        Self {
            minimal_vm_requirements,
            deadline,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            minimal_vm_requirements: Vm { cpu: 1, mem: 1000 },
            deadline: Utc::now(),
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deadline.partial_cmp(&other.deadline)
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deadline.cmp(&other.deadline)
    }
}
