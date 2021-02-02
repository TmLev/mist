use std::time::Duration;

use stakker::CX;

use crate::proto::InstanceTypesResponse;
use crate::providers::proto::InstanceTypesRequest;
use crate::vms::InstanceTypes;

#[derive(Debug)]
pub struct ExternalProvider {
    instance_types: InstanceTypes,
}

impl ExternalProvider {
    pub fn init(cx: CX![], instance_types: InstanceTypes) -> Option<Self> {
        Some(Self { instance_types })
    }

    pub fn instance_types(&self) -> InstanceTypes {
        self.instance_types.clone()
    }
}
