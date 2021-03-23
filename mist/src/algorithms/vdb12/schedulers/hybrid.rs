use super::queues::PriorityQueue;
use crate::vdb12::{
    policies::{CheapestToPublic, UnfeasiblePolicy, UnfeasibleToPublic},
    Application, InstanceType,
};

struct HybridScheduler {
    private_instance_types: Vec<InstanceType>,
    application_queue: Box<dyn PriorityQueue>,
    unfeasible_policy: Box<dyn UnfeasiblePolicy>,
}

impl HybridScheduler {
    pub fn new(
        private_instance_types: Vec<InstanceType>,
        application_queue: Box<dyn PriorityQueue>,
        unfeasible_policy: Box<dyn UnfeasiblePolicy>,
    ) -> Self {
        Self {
            private_instance_types,
            application_queue,
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
