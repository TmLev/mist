use actix::prelude::*;

use crate::vm::Vm;

#[derive(Message)]
#[rtype(result = "InstanceTypesResponse")]
pub struct InstanceTypesRequest(pub ());

pub struct InstanceTypesResponse {
    pub types: Vec<Vm>,
}
