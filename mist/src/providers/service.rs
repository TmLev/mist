use stakker::{call, ret_some_to, Actor, CX};

use crate::providers::external::ExternalProvider;
use crate::vms::InstanceTypes;

pub struct ServiceProvider {
    pub eps: Vec<Actor<ExternalProvider>>,
}

impl ServiceProvider {
    pub fn init(cx: CX![], eps: Vec<Actor<ExternalProvider>>) -> Option<ServiceProvider> {
        println!("SP: init start");
        let ep = eps[0].clone();
        println!("SP: before call!-ing request");
        call!([cx], request_instance_types(ep));
        println!("SP: after call!-ing request");
        Some(Self { eps })
    }

    pub fn request_instance_types(&self, cx: CX![], ep: Actor<ExternalProvider>) {
        println!("SP: requesting instance types, before ret_to!");
        let ret = ret_some_to!([cx], receive_instance_types() as (InstanceTypes));
        println!("SP: after ret_to!, before call!");
        call!([ep], instance_types(ret));
        println!("SP: after call!");
    }

    pub fn receive_instance_types(&self, _cx: CX![], its: InstanceTypes) {
        println!("{:?}", its);
    }
}
