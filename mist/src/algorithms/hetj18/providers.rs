use stakker::CX;

use crate::hetj18::{Job, PrivatePool, PublicPool};

pub struct ServiceProvider {
    private_pool: PrivatePool,
    public_cloud: Vec<PublicPool>,
}

impl ServiceProvider {
    pub fn init(
        _cx: CX![],
        private_pool: PrivatePool,
        public_cloud: Vec<PublicPool>,
    ) -> Option<Self> {
        log::info!("Initialising");
        Some(Self {
            private_pool,
            public_cloud,
        })
    }

    pub fn user_request(&mut self, _cx: CX![], mut jobs: Vec<Job>) {
        log::info!(
            "Received user request, job uuids: {}",
            jobs.iter()
                .map(|job| job.uuid().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        priority_sort(&mut jobs);

        for job in jobs.iter() {
            if self.private_pool.try_schedule(job) {
                continue;
            }

            if self.try_public_schedule(job) {
                continue;
            }
        }
    }

    fn try_public_schedule(&mut self, job: &Job) -> bool {
        self.public_cloud
            .iter()
            .any(|public_pool| public_pool.try_schedule(job))
    }
}

fn priority_sort(jobs: &mut Vec<Job>) {
    jobs.sort_by_key(|job| job.deadline());
}
