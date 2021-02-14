use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Task {
    deadline: Instant,
}
