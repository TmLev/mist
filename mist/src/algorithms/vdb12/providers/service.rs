use crate::vdb12::{Application, HybridScheduler};

pub struct ServiceProvider {
    scheduler: HybridScheduler,
}

impl ServiceProvider {
    pub fn init(_cx: CX![], scheduler: HybridScheduler) -> Option<Self> {
        Some(Self { scheduler })
    }

    pub fn customer_request(&mut self, _cx: CX![], applications: Vec<Application>) {
        self.scheduler.add_applications(applications);
    }
}
