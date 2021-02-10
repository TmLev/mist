use std::time::Duration;

use stakker::{after, ret, Ret, CX};

use crate::vms::InstanceTypes;

#[derive(Debug)]
pub struct ExternalProvider {
    instance_types: InstanceTypes,
}

impl ExternalProvider {
    pub fn init(_cx: CX![], instance_types: InstanceTypes) -> Option<Self> {
        println!("EP: init start");
        Some(Self { instance_types })
    }

    pub fn instance_types(&self, cx: CX![], ret: Ret<InstanceTypes>) {
        println!("EP: before after!-ing");
        after!(Duration::from_secs(50), [cx], send_response(ret));
        println!("EP: after after!-ing");
    }

    fn send_response(&self, _cx: CX![], ret: Ret<InstanceTypes>) {
        println!("EP::send_response: before ret!-ing");
        ret!([ret], self.instance_types.clone());
        println!("EP::send_response: after ret!-ing");
    }
}
