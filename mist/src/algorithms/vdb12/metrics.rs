use crate::vdb12::Application;

pub struct Metrics {
    pub num_missed_deadlines: usize,
}

pub struct Galactus {
    missed_deadline_applications: Vec<Application>,
}

impl Galactus {
    pub fn new() -> Self {
        Self {
            missed_deadline_applications: Vec::new(),
        }
    }

    pub fn missed_deadline(&mut self, application: Application) {
        log::info!(
            "Application {} deadline can not be met",
            application.uuid().to_string()
        );
        self.missed_deadline_applications.push(application);
    }

    pub fn metrics(&mut self) -> Metrics {
        Metrics {
            num_missed_deadlines: self.missed_deadline_applications.len(),
        }
    }
}
