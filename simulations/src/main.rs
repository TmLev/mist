use std::time::{Duration, Instant};

use stakker::{actor, after, call, ret, ret_nop, ret_some_to, Actor, Ret, Stakker, CX};

use mist::rusty_money::{iso::Currency, iso::USD, Money};

#[derive(Debug, Clone)]
struct Vm {
    cpu: u32,
    ram: u32,
}

impl Vm {
    fn new(cpu: u32, ram: u32) -> Self {
        Self { cpu, ram }
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self { cpu: 4, ram: 2048 }
    }
}

type Cost = Money<'static, Currency>;

#[derive(Debug, Clone)]
struct InstanceType {
    vm: Vm,
    cost: Cost,
}

impl InstanceType {
    pub fn new(vm: Vm, cost: Cost) -> Self {
        Self { vm, cost }
    }
}

type InstanceTypes = Vec<InstanceType>;

#[derive(Debug)]
struct ExternalProvider {
    instance_types: InstanceTypes,
}

impl ExternalProvider {
    fn init(_cx: CX![], instance_types: InstanceTypes) -> Option<Self> {
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

struct ServiceProvider {
    eps: Vec<Actor<ExternalProvider>>,
}

impl ServiceProvider {
    fn init(cx: CX![], eps: Vec<Actor<ExternalProvider>>) -> Option<Self> {
        println!("SP: init start");
        let ep = eps[0].clone();
        println!("SP: before call!-ing request");
        call!([cx], request_instance_types(ep));
        println!("SP: after call!-ing request");
        Some(Self { eps })
    }

    fn request_instance_types(&self, cx: CX![], ep: Actor<ExternalProvider>) {
        println!("SP: requesting instance types, before ret_to!");
        let ret = ret_some_to!([cx], receive_instance_types() as (InstanceTypes));
        println!("SP: after ret_to!, before call!");
        call!([ep], instance_types(ret));
        println!("SP: after call!");
    }

    fn receive_instance_types(&self, _cx: CX![], its: InstanceTypes) {
        println!("{:?}", its);
    }
}

fn main() -> () {
    let mut stakker0 = Stakker::new(Instant::now());
    let stakker = &mut stakker0;

    let ep = actor!(
        stakker,
        ExternalProvider::init(vec![
            InstanceType::new(Vm::new(2, 1024), Money::from_major(10, USD)),
            InstanceType::new(Vm::new(4, 2048), Money::from_major(20, USD)),
            InstanceType::new(Vm::new(8, 2048), Money::from_major(30, USD)),
            InstanceType::new(Vm::new(8, 4096), Money::from_major(40, USD)),
        ]),
        ret_nop!()
    );
    let _sp = actor!(stakker, ServiceProvider::init(vec![ep.clone()]), ret_nop!());

    let mut count = 0;
    let mut now = Instant::now();

    stakker.run(now, false);
    while stakker.not_shutdown() {
        now += stakker.next_wait_max(now, Duration::from_secs(10), false);
        stakker.run(now, false);

        count += 1;
        if count > 50 {
            break;
        }
    }
}
