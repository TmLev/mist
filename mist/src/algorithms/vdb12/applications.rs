use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::algorithms::vdb12::tasks::Task;

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub enum Status {
    Schedulable,
    Unfeasible,
}

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Application {
    uuid: Uuid,
    status: Status,
    tasks: Vec<Task>,
}

impl Application {
    pub fn new(uuid: Uuid, status: Status, tasks: Vec<Task>) -> Self {
        Self {
            uuid,
            status,
            tasks,
        }
    }

    pub fn add_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks.extend(tasks);
        self.tasks.sort();
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            status: Status::Schedulable,
            tasks: vec![],
        }
    }
}
