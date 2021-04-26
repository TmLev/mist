use std::time::{Duration, Instant};

use stakker::{after, call, stop, CX};

use crate::hetj18::Job;

pub struct User {
    start_time: Instant,
    num_total_requests: usize,
    num_scheduled_requests: usize,
}

impl User {
    pub fn init(cx: CX![]) -> Option<Self> {
        log::info!("Initialising");

        // Schedule first request.
        call!([cx], schedule_next_request());

        Some(Self {
            start_time: cx.now(),
            num_total_requests: 2,
            num_scheduled_requests: 0,
        })
    }

    pub fn schedule_next_request(&mut self, cx: CX![]) {
        if self.num_total_requests > 0 {
            // TODO(TmLev): customize (with closure?).
            let send_interval = Duration::new(10, 0);
            after!(send_interval, [cx], send_jobs());

            self.num_total_requests -= 1;
            self.num_scheduled_requests += 1;
        }

        if self.num_total_requests == 0 && self.num_scheduled_requests == 0 {
            stop!(cx);
        }
    }

    pub fn send_jobs(&mut self, cx: CX![]) {
        let jobs = self.generate_jobs();
    }

    fn generate_jobs(&self) -> Vec<Job> {
        vec![Job::generate()]
    }
}
