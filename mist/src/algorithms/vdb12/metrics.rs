use crate::vdb12::Application;

pub struct Metrics {
    num_missed_deadlines: usize,
}

pub struct Galactus {
    applications_with_missed_deadlines: Vec<Application>,
}

impl Galactus {
    pub fn new() -> Self {
        Self {
            applications_with_missed_deadlines: Vec::new(),
        }
    }

    pub fn missed_deadline(&mut self, application: Application) {
        log::info!(
            "Application {} deadline can not be met",
            application.uuid().to_string()
        );
        self.applications_with_missed_deadlines.push(application);
    }

    pub fn metrics(&mut self) -> Metrics {
        Metrics {
            num_missed_deadlines: self.applications_with_missed_deadlines.len(),
        }
    }
}
