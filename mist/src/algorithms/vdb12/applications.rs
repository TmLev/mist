use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::vdb12::Task;

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Application {
    pub uuid: Uuid,
    pub tasks: Vec<Task>,
}

impl Application {
    pub fn new(uuid: Uuid, tasks: Vec<Task>) -> Self {
        Self { uuid, tasks }
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
            tasks: vec![],
        }
    }
}
