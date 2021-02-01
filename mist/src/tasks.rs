pub type Deadline = i64;

#[derive(Debug, Clone)]
pub struct Task {
    deadline: Deadline,
}

pub type Tasks = Vec<Task>;
