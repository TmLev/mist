use chrono::{DateTime, Utc};

pub struct Task {}

pub type Deadline = DateTime<Utc>;

pub struct Job {
    tasks: Vec<Task>,
    deadline: Deadline,
}

impl Job {
    pub fn generate() -> Self {
        todo!()
    }
}
