use actix::prelude::*;

use crate::vm::Vm;
use crate::providers::proto::{
    Request,
};

#[derive(Debug)]
pub struct ExternalProvider {
    instance_types: Vec<Vm>,
}

impl ExternalProvider {
    pub fn new(instance_types: Vec<Vm>) -> ExternalProvider {
        ExternalProvider { instance_types }
    }

    pub fn instance_types(&self) -> &[Vm] {
        &self.instance_types
    }
}

impl Actor for ExternalProvider {
    type Context = Context<Self>;
}

impl Handler<Request> for ExternalProvider {
    type Result = usize;

    fn handle(&mut self, msg: Request, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + 1
    }
}
