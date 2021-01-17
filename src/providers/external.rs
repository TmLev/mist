use actix::prelude::*;

use crate::vm::Vm;
use crate::providers::proto::InstanceTypesRequest;
use crate::proto::InstanceTypesResponse;
use actix::dev::MessageResponse;

#[derive(Debug)]
pub struct ExternalProvider {
    instance_types: Vec<Vm>,
}

impl ExternalProvider {
    pub fn new(instance_types: Vec<Vm>) -> ExternalProvider {
        ExternalProvider { instance_types }
    }

    pub fn instance_types(&self) -> Vec<Vm> {
        self.instance_types.clone()
    }
}

impl Actor for ExternalProvider {
    type Context = Context<Self>;
}

impl Handler<InstanceTypesRequest> for ExternalProvider {
    type Result = MessageResult<InstanceTypesRequest>;

    fn handle(&mut self, msg: InstanceTypesRequest, _ctx: &mut Context<Self>) -> Self::Result {
        MessageResult(InstanceTypesResponse { types: self.instance_types() })
    }
}
