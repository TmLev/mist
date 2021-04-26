use std::time::Instant;

use crate::hetj18::Job;

#[derive(Debug, Clone)]
struct Record<T> {
    pub time: Instant,
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    missed_deadline_jobs: Vec<Record<Job>>,
    // public_scheduling_costs: Vec<Record<Cost>>,
    // utilisation_levels: Vec<Record<Utilisation>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            missed_deadline_jobs: Vec::new(),
        }
    }

    pub fn missed_deadline(&mut self, now: Instant, job: Job) {
        log::info!("Job {} deadline can not be met", job.uuid().to_string());
        self.missed_deadline_jobs.push(Record {
            time: now,
            value: job,
        });
    }

    pub fn num_missed_deadlines(&self) -> usize {
        self.missed_deadline_jobs.len()
    }
}
