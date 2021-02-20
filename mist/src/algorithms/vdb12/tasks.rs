use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Task {
    deadline: DateTime<Utc>,
}

impl Task {
    pub fn new(deadline: DateTime<Utc>) -> Self {
        Self { deadline }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
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
