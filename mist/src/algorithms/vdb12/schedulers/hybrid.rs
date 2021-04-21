use std::time::Instant;

use uuid::Uuid;

use crate::vdb12::{Application, Galactus, InstanceType, PublicScheduler};

/// Sorting policy determines in what order applications waiting in queue should be scheduled.
pub enum SortingPolicy {
    /// FIFO order.
    FirstComeFirstServed,
    /// Prioritise applications with the earliest deadline.
    EarliestDeadlineFirst,
}

/// Unfeasible application policy.
pub enum UnfeasiblePolicy {
    /// Find the cheapest application before the unfeasible one and delegate it.
    CheapestToPublic,
    /// Send the unfeasible application to the public cloud.
    UnfeasibleToPublic,
}

/// Hybrid cloud scheduler. Distributes arriving applications between the private and public clouds.
pub struct HybridScheduler {
    /// Current simulation time.
    now: Instant,

    /// Queue for incoming applications.
    application_queue: Vec<Application>,
    /// Queue sorting policy.
    sorting_policy: SortingPolicy,
    /// Unfeasible applications policy.
    unfeasible_policy: UnfeasiblePolicy,

    /// Restrictions of the private cloud.
    max_cpu: u32,
    max_mem: u32,
    /// Available instance types in the private cloud.
    private_instance_types: Vec<InstanceType>,

    /// The public cloud scheduler.
    public_scheduler: PublicScheduler,

    /// Metrics watcher.
    galactus: Galactus,
}

impl HybridScheduler {
    pub fn new(
        now: Instant,

        sorting_policy: SortingPolicy,
        unfeasible_policy: UnfeasiblePolicy,

        private_instance_types: Vec<InstanceType>,

        public_scheduler: PublicScheduler,
    ) -> Self {
        Self {
            now,

            application_queue: Vec::new(),
            sorting_policy,
            unfeasible_policy,

            // TODO(TmLev): extract & customize.
            max_cpu: 60,
            max_mem: 64000,
            private_instance_types,

            public_scheduler,
            galactus: Galactus::new(),
        }
    }

    pub fn advance_time(&mut self, now: Instant) {
        self.now = now;
    }

    pub fn add_applications(&mut self, applications: Vec<Application>) {
        self.application_queue.extend(applications);
        self.sort_queue();
    }

    fn remove_application_from_queue(&mut self, uuid: Uuid) -> Application {
        let index = self
            .application_queue
            .iter()
            .position(|application| application.uuid() == uuid)
            .unwrap();
        self.application_queue.remove(index)
    }

    fn sort_queue(&mut self) {
        match self.sorting_policy {
            SortingPolicy::FirstComeFirstServed => {}
            SortingPolicy::EarliestDeadlineFirst => self
                .application_queue
                .sort_by_key(|application| application.deadline()),
        };
    }

    pub fn scan(&mut self) {
        // TODO(TmLev): this is an example of detecting unfeasible application.
        if self.application_queue.len() > 0 {
            let uuid = self.application_queue[0].uuid();
            let unfeasible = self.remove_application_from_queue(uuid);
            self.apply_unfeasible_policy(unfeasible);
        }
    }

    pub fn schedule(&mut self) {}

    pub fn apply_unfeasible_policy(&mut self, unfeasible: Application) {
        match self.unfeasible_policy {
            UnfeasiblePolicy::UnfeasibleToPublic => {
                let cheapest_public_provider = self
                    .public_scheduler
                    .cheapest_public_provider(&unfeasible, self.now);
                match cheapest_public_provider {
                    None => {
                        // TODO(TmLev): application deadline can not be met.
                        self.galactus.missed_deadline(unfeasible);
                    }
                    Some(public_provider) => public_provider.schedule(unfeasible),
                }
            }
            UnfeasiblePolicy::CheapestToPublic => {}
        }
    }
}
