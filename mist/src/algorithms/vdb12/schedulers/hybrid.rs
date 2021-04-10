use crate::vdb12::{Application, InstanceType};

pub enum SortingPolicy {
    FirstComeFirstServed,
    EarliestDeadlineFirst,
}

pub enum UnfeasiblePolicy {
    CheapestToPublic,
    UnfeasibleToPublic,
}

pub struct HybridScheduler {
    private_instance_types: Vec<InstanceType>,
    application_queue: Vec<Application>,
    sorting_policy: SortingPolicy,
    unfeasible_policy: UnfeasiblePolicy,
}

impl HybridScheduler {
    pub fn new(
        private_instance_types: Vec<InstanceType>,
        sorting_policy: SortingPolicy,
        unfeasible_policy: UnfeasiblePolicy,
    ) -> Self {
        Self {
            private_instance_types,
            application_queue: Vec::new(),
            sorting_policy,
            unfeasible_policy,
        }
    }

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

    fn scan(&mut self) -> Option<Application> {
        for (index, application) in self.application_queue.iter_mut().enumerate() {
            for task in &application.tasks.iter() {
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
