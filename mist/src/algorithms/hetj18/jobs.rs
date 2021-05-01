use std::time::Instant;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {}

pub type Deadline = Instant;

#[derive(Debug, Clone)]
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

    pub fn deadline(&self) -> Deadline {
        self.deadline
    }
}
