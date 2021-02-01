use std::time::Duration;

use stakker;
use stakker::Stakker;

use crate::proto::InstanceTypesResponse;
use crate::providers::proto::InstanceTypesRequest;
use crate::vms::Vm;

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
    pub fn init(cx: CX![], instance_types: InstanceTypes) -> Option<Self> {
        cx
        Some(Self { instance_types })
    }

    pub fn instance_types(&self) -> InstanceTypes {
        self.instance_types.clone()
    }
}

impl Handler<InstanceTypesRequest> for ExternalProvider {
    type Result = MessageResult<InstanceTypesRequest>;

    fn handle(&mut self, _msg: InstanceTypesRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let mut stakker = Stakker::new(std::time::Instant::now());
        let s = &mut stakker;

        std::thread::sleep(Duration::from_secs(2));
        MessageResult(InstanceTypesResponse(self.instance_types()))
    }
}
