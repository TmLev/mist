use std::time::Duration;

use chrono::{DateTime, Utc};
use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::vdb12::VirtualMachine;

#[derive(Debug, Clone)]
pub struct Task {
    pub requirements: VirtualMachine,
    pub runtime: Duration,
}

impl Task {
    pub fn generate(base_runtime: Duration) -> Self {
        Self {
            // TODO(TmLev): customize.
            requirements: VirtualMachine { cpu: 1, mem: 1000 },
            runtime: base_runtime,
        }
    }
}

pub type Deadline = DateTime<Utc>;

#[derive(Debug, Clone)]
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
            tasks,
            uuid: Uuid::new_v4(),
            deadline,
        }
    }

    // Builder pattern.
    pub fn with_uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = uuid;
        self
    }

    pub fn generate() -> Self {
        let num_tasks = thread_rng().gen_range(1..10);
        log::debug!("Generating new application with {} tasks", num_tasks);

        let weibull = rand_distr::Weibull::new(1879.0, 0.426).unwrap();
        let base_runtime = thread_rng().sample(weibull) as u64;
        let base_runtime = Duration::from_secs(3600 * base_runtime);

        Self {
            uuid: Uuid::new_v4(),
            tasks: (0..num_tasks)
                .map(|_| Task::generate(base_runtime))
                .collect(),
            deadline: Utc::now(), // FIXME(TmLev): wrong.
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Accessors

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn deadline(&self) -> Deadline {
        self.deadline
    }
}

impl PartialEq for Application {
    fn eq(&self, other: &Application) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Application {}
