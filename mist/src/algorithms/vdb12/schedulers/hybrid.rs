use std::time::Instant;

use crate::vdb12::{Application, InstanceType, PublicScheduler};

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
    /// Available instance types in the private cloud.
    private_instance_types: Vec<InstanceType>,
    /// Queue for incoming applications.
    application_queue: Vec<Application>,
    /// Queue sorting policy.
    sorting_policy: SortingPolicy,
    /// Unfeasible applications policy.
    unfeasible_policy: UnfeasiblePolicy,
    /// The public cloud scheduler.
    public_scheduler: PublicScheduler,
}

impl HybridScheduler {
    pub fn new(
        private_instance_types: Vec<InstanceType>,
        sorting_policy: SortingPolicy,
        unfeasible_policy: UnfeasiblePolicy,
        public_scheduler: PublicScheduler,
    ) -> Self {
        Self {
            private_instance_types,
            application_queue: Vec::new(),
            sorting_policy,
            unfeasible_policy,
            public_scheduler,
        }
    }

    pub fn add_applications(&mut self, applications: Vec<Application>) {
        self.application_queue.extend(applications);
        self.sort_queue();
    }

    fn remove_application_from_queue(&mut self, to_remove: &Application) -> Application {
        let application = to_remove.clone();
        // `Vec::retain` preserves order of elements, no need to sort.
        self.application_queue
            .retain(|application| application != to_remove);
        application
    }

    fn sort_queue(&mut self) {
        match self.sorting_policy {
            SortingPolicy::FirstComeFirstServed => {}
            SortingPolicy::EarliestDeadlineFirst => self
                .application_queue
                .sort_by_key(|application| application.deadline()),
        };
    }

    pub fn scan(&mut self, now: Instant) {
        for (index, application) in self.application_queue.iter_mut().enumerate() {
            let status = { for task in application.tasks.iter() {} };
        }
    }

    pub fn schedule(&mut self, now: Instant) {}

    fn apply_unfeasible_policy(&mut self, unfeasible_application: &Application) {
        let unfeasible_application = self.remove_application_from_queue(unfeasible_application);

        match self.unfeasible_policy {
            UnfeasiblePolicy::UnfeasibleToPublic => {
                let cheapest_public_provider = self
                    .public_scheduler
                    .cheapest_public_provider(&unfeasible_application);
                match cheapest_public_provider {
                    None => {
                        // TODO(TmLev): application deadline can not be met.
                    }
                    Some(public_provider) => public_provider.schedule(unfeasible_application),
                }
            }
            UnfeasiblePolicy::CheapestToPublic => {}
        }
    }
}
