use std::collections::VecDeque;

use crate::vdb12::{Application, InstanceType};

enum QueueSortingPolicy {
    FirstComeFirstServed,
    EarliestDeadlineFirst,
}

enum UnfeasiblePolicy {
    CheapestToPublic,
    UnfeasibleToPublic,
}

struct HybridScheduler {
    private_instance_types: Vec<InstanceType>,
    application_queue: VecDeque<Application>,
    queue_sorting_policy: QueueSortingPolicy,
    unfeasible_policy: UnfeasiblePolicy,
}

impl HybridScheduler {
    pub fn new(
        private_instance_types: Vec<InstanceType>,
        queue_sorting_policy: QueueSortingPolicy,
        unfeasible_policy: UnfeasiblePolicy,
    ) -> Self {
        Self {
            private_instance_types,
            application_queue: VecDeque::new(),
            queue_sorting_policy,
            unfeasible_policy,
        }
    }

    pub fn enqueue_application(&mut self, application: Application) {
        ap
    }

    fn scan(&mut self) -> Option<Application> {
        for (index, application) in self.applications.iter_mut().enumerate() {
            for task in &application.tasks {
                let time = get_start_time(schedule, task);
                let it = get_instance_type(task, time);

                match it {
                    Some(it) => {}
                    None => {
                        self.policy(application);
                        break;
                    }
                }
            }
        }

        None
    }

    fn apply_policy(&mut self, unfeasible_application: &Application) {}
}
