use std::time::Duration;

use stakker::{Actor, CX};

use crate::vdb12::{Application, ServiceProvider};

pub struct Customer {
    service_provider: Actor<ServiceProvider>,
    send_interval: Duration,
}

impl Customer {
    pub fn init(cx: CX![], service_provider: Actor<ServiceProvider>) -> Option<Self> {
        // Schedule first request.
        call!([cx], schedule_next_request());

        Some(Self {
            service_provider,
            send_interval,
        })
    }

    fn schedule_next_request(&self, cx: CX![]) {
        // TODO(TmLev): randomize.
        let send_interval = Duration::new(100, 0);
        after!(send_interval, [cx], send_applications());
    }

    pub fn send_applications(&self, cx: CX![]) {
        let applications = self.generate_applications();

        // Send applications to service provider.
        call!([self.service_provider], customer_request(applications));

        // Repeat again after some time.
        call!([cx], schedule_next_request());
    }

    fn generate_applications(&self) -> Vec<Application> {
        todo!()
    }
}
