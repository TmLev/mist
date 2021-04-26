use std::{cell::RefCell, rc::Rc, time::Instant};

use uuid::Uuid;

use crate::vdb12::{Application, Metrics, PrivateScheduler, PublicScheduler};

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

    /// The private cloud scheduler.
    private_scheduler: PrivateScheduler,
    /// The public cloud scheduler.
    public_scheduler: PublicScheduler,

    /// Metrics watcher.
    metrics: Rc<RefCell<Metrics>>,
}

impl HybridScheduler {
    pub fn new(
        now: Instant,

        sorting_policy: SortingPolicy,
        unfeasible_policy: UnfeasiblePolicy,

        private_scheduler: PrivateScheduler,
        public_scheduler: PublicScheduler,

        metrics: Rc<RefCell<Metrics>>,
    ) -> Self {
        Self {
            now,

            application_queue: Vec::new(),
            sorting_policy,
            unfeasible_policy,

            private_scheduler,
            public_scheduler,

            metrics,
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Queue

    pub fn add_applications(&mut self, applications: Vec<Application>) {
        self.application_queue.extend(applications);
        self.sort_queue();
    }

    fn sort_queue(&mut self) {
        match self.sorting_policy {
            SortingPolicy::FirstComeFirstServed => {}
            SortingPolicy::EarliestDeadlineFirst => self
                .application_queue
                .sort_by_key(|application| application.deadline()),
        };
    }

    fn find_application_index_in_queue(&mut self, uuid: Uuid) -> Option<usize> {
        self.application_queue
            .iter()
            .position(|application| application.uuid() == uuid)
    }

    fn remove_application_from_queue(&mut self, uuid: Uuid) -> Application {
        let index = self.find_application_index_in_queue(uuid).unwrap();
        self.application_queue.remove(index)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Simulation

    pub fn advance_time(&mut self, now: Instant) {
        self.now = now;
    }

    pub fn scan(&mut self) {
        // TODO(TmLev): this is an example of detecting unfeasible application.
        if self.application_queue.len() > 0 {
            self.apply_unfeasible_policy(self.application_queue[0].uuid());
        }
    }

    pub fn schedule(&mut self) {
        if self.application_queue.is_empty() {
            log::debug!("Has nothing to schedule");
            return;
        }

        let candidate = self.application_queue[0].clone();
        let uuid = candidate.uuid();
        log::debug!("Trying to schedule {} on private", uuid);

        if self.private_scheduler.try_schedule(candidate) {
            self.remove_application_from_queue(uuid);
        }
    }

    fn apply_unfeasible_policy(&mut self, unfeasible_uuid: Uuid) {
        // TODO(TmLev): application deadline can not be met.

        match self.unfeasible_policy {
            UnfeasiblePolicy::UnfeasibleToPublic => {
                let unfeasible = self.remove_application_from_queue(unfeasible_uuid);
                let cheapest_public_provider = self
                    .public_scheduler
                    .cheapest_public_provider(&unfeasible, self.now);
                match cheapest_public_provider {
                    None => self
                        .metrics
                        .borrow_mut()
                        .missed_deadline(self.now, unfeasible),
                    Some(uuid) => self.public_scheduler.schedule_on(unfeasible, uuid),
                }
            }

            UnfeasiblePolicy::CheapestToPublic => {
                let unfeasible_index = self
                    .find_application_index_in_queue(unfeasible_uuid)
                    .unwrap();
                let unfeasible_deadline = self.application_queue[unfeasible_index].deadline();

                let next_after_unfeasible: Option<usize> = self
                    .application_queue
                    .iter()
                    .position(|application| application.deadline() >= unfeasible_deadline);

                match next_after_unfeasible {
                    None => {
                        let unfeasible = self.remove_application_from_queue(unfeasible_uuid);
                        self.metrics
                            .borrow_mut()
                            .missed_deadline(self.now, unfeasible);
                    }
                    Some(index) => {
                        let cheapest_application: Option<&Application> = None;

                        match cheapest_application {
                            None => {
                                let unfeasible =
                                    self.remove_application_from_queue(unfeasible_uuid);
                                self.metrics
                                    .borrow_mut()
                                    .missed_deadline(self.now, unfeasible);
                            }
                            Some(application) => {
                                let application =
                                    self.remove_application_from_queue(application.uuid());
                                self.public_scheduler.schedule(application);
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_application_to_delegate(&mut self) {}
}
