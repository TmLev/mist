use std::time::{Duration, Instant};

use stakker::{after, call, stop, Actor, CX};

use crate::vdb12::{Application, ServiceProvider};

pub struct Customer {
    start_time: Instant,
    service_provider: Actor<ServiceProvider>,
    num_total_requests: usize,
    num_scheduled_requests: usize,
}

impl Customer {
    pub fn init(cx: CX![], service_provider: Actor<ServiceProvider>) -> Option<Self> {
        log::info!("Initialising");

        // Schedule first request.
        call!([cx], schedule_next_request());

        Some(Self {
            start_time: cx.now(),
            service_provider,
            num_total_requests: 20, // TODO(TmLev): customize (with closure?).
            num_scheduled_requests: 0,
        })
    }

    fn schedule_next_request(&mut self, cx: CX![]) {
        if self.num_total_requests > 0 {
            // TODO(TmLev): customize (with closure?).
            let send_interval = Duration::new(10, 0);
            after!(send_interval, [cx], send_applications());

            self.num_total_requests -= 1;
            self.num_scheduled_requests += 1;
        }

        if self.num_total_requests == 0 && self.num_scheduled_requests == 0 {
            stop!(cx);
        }
    }

    pub fn send_applications(&mut self, cx: CX![]) {
        let applications = self.generate_applications();

        log::info!(
            "[T {}] Sending applications {:?}",
            (cx.now() - self.start_time).as_secs(),
            applications
        );

        // Send applications to service provider.
        call!([self.service_provider], customer_request(applications));
        self.num_scheduled_requests -= 1;

        // Repeat again after some time.
        call!([cx], schedule_next_request());
    }

    fn generate_applications(&self) -> Vec<Application> {
        vec![Application::gen()]
    }
}
