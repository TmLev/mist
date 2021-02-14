use crate::tasks::Tasks;

#[derive(Debug, Clone)]
pub enum ApplicationStatus {
    Schedulable,
    Unfeasible,
}

#[derive(Debug, Clone)]
pub struct Application {
    tasks: Tasks,
    status: ApplicationStatus,
}
