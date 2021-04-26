use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::ops::Deref;
use uuid::Uuid;

pub struct Task {}

pub type Deadline = DateTime<Utc>;

pub struct Job {
    uuid: Uuid,
    tasks: Vec<Task>,
    deadline: Deadline,
}

impl Job {
    pub fn generate() -> Self {
        todo!()
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}
