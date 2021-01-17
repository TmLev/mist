use actix::prelude::*;

use crate::providers::external::ExternalProvider;
use crate::Vm;
use crate::proto::InstanceTypesRequest;

#[derive(Debug)]
pub struct ServiceProvider {
     pub eps: Vec<Addr<ExternalProvider>>,
}

impl ServiceProvider {
     pub fn new(eps: Vec<Addr<ExternalProvider>>) -> ServiceProvider {
          ServiceProvider { eps }
     }

     pub async fn request_instance_types(&self, ep_addr: &Addr<ExternalProvider>) -> Vec<Vm> {
          ep_addr.send(InstanceTypesRequest(())).await.unwrap().types
     }
}
