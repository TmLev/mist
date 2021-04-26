use chrono::{DateTime, Utc};

use crate::hetj18::Resource;

pub struct Task {}

pub type Deadline = DateTime<Utc>;

pub struct Job {
    tasks: Vec<Task>,
    deadline: Deadline,
}

impl Job {}
