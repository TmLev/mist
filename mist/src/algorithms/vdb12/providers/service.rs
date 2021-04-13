use log;

use stakker::CX;

use crate::vdb12::{Application, HybridScheduler};

pub struct ServiceProvider {
    scheduler: HybridScheduler,
}

impl ServiceProvider {
    pub fn init(_cx: CX![], scheduler: HybridScheduler) -> Option<Self> {
        log::info!("Initializing");
        Some(Self { scheduler })
    }

    pub fn customer_request(&mut self, _cx: CX![], applications: Vec<Application>) {
        log::info!("Customer request arrived");
        self.scheduler.add_applications(applications);
    }
}
