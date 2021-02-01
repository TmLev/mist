use crate::tasks::Tasks;

#[derive(Debug, Clone)]
enum ApplicationStatus {
    Schedulable,
    Unfeasible,
}

#[derive(Debug, Clone)]
struct Application {
    tasks: Tasks,
    status: ApplicationStatus,
}
