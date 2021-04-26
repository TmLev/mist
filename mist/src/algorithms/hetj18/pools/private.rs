use crate::hetj18::{Job, Node};

pub struct PrivatePool {
    nodes: Vec<Node>,
}

impl PrivatePool {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn try_schedule(&mut self, job: &Job) -> bool {
        todo!()
    }
}
