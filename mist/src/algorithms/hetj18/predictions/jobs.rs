use std::time::Duration;

use rand::Rng;

use crate::hetj18::{Job, Resource, Task};

/// Should be replaced with ML models, but there is no time for that right now.
pub fn predict_job_intensity(_job: &Job) -> Resource {
    let zero_to_one: f32 = rand::random();

    if zero_to_one <= 0.33 {
        Resource::Cpu
    } else if zero_to_one <= 0.66 {
        Resource::Io
    } else {
        Resource::Both
    }
}

pub fn predict_task_execution_time(_task: &Task) -> Duration {
    let secs = rand::thread_rng().gen_range(50..100);
    Duration::from_secs(secs)
}
