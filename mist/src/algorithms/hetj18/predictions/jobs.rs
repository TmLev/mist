use crate::hetj18::{Job, Resource};

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
