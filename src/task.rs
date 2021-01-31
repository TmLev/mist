pub type Deadline = i64;

#[derive(Debug, Clone)]
struct Task {
    deadline: Deadline,
}

pub type Tasks = Vec<Task>;

enum ApplicationStatus {
    Schedulable,
    Unfeasible,
}

#[derive(Debug, Clone)]
struct Application {
    tasks: Tasks,
    status: ApplicationStatus,
}
