use std::time::Duration;

use log;

use stakker::{after, CX};

use crate::vdb12::{Application, HybridScheduler};

pub struct ServiceProvider {
    scheduler: HybridScheduler,
    scan_interval: Duration,
    schedule_interval: Duration,
}

impl ServiceProvider {
    pub fn init(cx: CX![], scheduler: HybridScheduler) -> Option<Self> {
        log::info!("Initialising");

        // TODO(TmLev): customize.
        let scan_interval = Duration::from_secs(5);
        let schedule_interval = Duration::from_secs(1);

        after!(scan_interval, [cx], scan());
        after!(schedule_interval, [cx], schedule());

        Some(Self {
            scheduler,
            scan_interval,
            schedule_interval,
        })
    }

    pub fn customer_request(&mut self, cx: CX![], applications: Vec<Application>) {
        log::info!(
            "Customer request arrived, application uuids: [{}]",
            applications
                .clone()
                .iter()
                .map(|application| application.uuid().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        for application in applications.iter() {}
        self.scheduler.advance_time(cx.now());
        self.scheduler.add_applications(applications);
    }

    fn scan(&mut self, cx: CX![]) {
        log::debug!("Scanning application queue");
        self.scheduler.advance_time(cx.now());
        self.scheduler.scan();
        after!(self.scan_interval, [cx], scan());
    }

    fn schedule(&mut self, cx: CX![]) {
        log::debug!("Scheduling applications");
        self.scheduler.advance_time(cx.now());
        self.scheduler.schedule();
        after!(self.schedule_interval, [cx], schedule());
    }
}
