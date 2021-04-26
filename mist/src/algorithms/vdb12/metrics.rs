use std::time::Instant;

use crate::vdb12::{Application, Cost, Utilisation};

struct Record<T> {
    pub time: Instant,
    pub value: T,
}

pub struct Metrics {
    missed_deadline_applications: Vec<Record<Application>>,
    public_scheduling_costs: Vec<Record<Cost>>,
    utilisation_levels: Vec<Record<Utilisation>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            missed_deadline_applications: Vec::new(),
            public_scheduling_costs: Vec::new(),
            utilisation_levels: Vec::new(),
        }
    }

    pub fn missed_deadline(&mut self, now: Instant, application: Application) {
        log::info!(
            "Application {} deadline can not be met",
            application.uuid().to_string()
        );
        self.missed_deadline_applications.push(Record {
            time: now,
            value: application,
        });
    }

    pub fn public_schedule(&mut self, now: Instant, cost: Cost) {
        log::info!("Public cloud schedule at cost of {}", cost);
        self.public_scheduling_costs.push(Record {
            time: now,
            value: cost,
        });
    }

    pub fn current_utilisation(&mut self, now: Instant, level: Utilisation) {
        log::info!("Current utilisation level is ~{}%", level.round());
        self.utilisation_levels.push(Record {
            time: now,
            value: level,
        });
    }

    pub fn num_missed_deadlines(&self) -> usize {
        self.missed_deadline_applications.len()
    }
}
