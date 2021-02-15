use crate::algorithms::vdb12::tasks::Task;

#[derive(Debug, Clone)]
pub enum Status {
    Schedulable,
    Unfeasible,
}

#[derive(Debug, Clone)]
pub struct Application {
    tasks: Vec<Task>,
    status: Status,
}
