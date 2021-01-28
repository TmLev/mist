use actix::prelude::*;

use crate::providers::external::InstanceTypes;

#[derive(Message)]
#[rtype(result = "InstanceTypesResponse")]
pub struct InstanceTypesRequest(pub ());

pub struct InstanceTypesResponse(pub InstanceTypes);
