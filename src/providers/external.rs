use actix::prelude::*;

use crate::proto::InstanceTypesResponse;
use crate::providers::proto::InstanceTypesRequest;
use crate::vm::Vm;
use std::time::Duration;

pub type Cost = f64;

#[derive(Debug, Clone)]
pub struct InstanceType {
    vm: Vm,
    cost: Cost,
}

impl InstanceType {
    pub fn new(vm: Vm, cost: Cost) -> Self {
        Self { vm, cost }
    }
}

pub type InstanceTypes = Vec<InstanceType>;

#[derive(Debug)]
pub struct ExternalProvider {
    instance_types: InstanceTypes,
}

impl ExternalProvider {
    pub fn new(instance_types: InstanceTypes) -> ExternalProvider {
        ExternalProvider { instance_types }
    }

    pub fn instance_types(&self) -> InstanceTypes {
        self.instance_types.clone()
    }
}

impl Actor for ExternalProvider {
    type Context = Context<Self>;
}

impl Handler<InstanceTypesRequest> for ExternalProvider {
    type Result = MessageResult<InstanceTypesRequest>;

    fn handle(&mut self, _msg: InstanceTypesRequest, _ctx: &mut Context<Self>) -> Self::Result {
        std::thread::sleep(Duration::from_secs(2));
        MessageResult(InstanceTypesResponse(self.instance_types()))
    }
}
