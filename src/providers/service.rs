use actix::prelude::*;

use crate::providers::external::ExternalProvider;
use crate::providers::proto::Request;

#[derive(Debug)]
pub struct ServiceProvider {
     eps: Vec<Addr<ExternalProvider>>,
}

impl ServiceProvider {
     pub fn new(eps: Vec<Addr<ExternalProvider>>) -> ServiceProvider {
          ServiceProvider { eps }
     }

     pub async fn request(&self) -> usize {
          let first = &self.eps[0];
          let res = first.send(Request(123usize)).await;
          res.unwrap()
     }
}
