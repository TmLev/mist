use actix::prelude::*;

use mist::{ExternalProvider, InstanceType, ServiceProvider, Vm};

#[actix_rt::main]
async fn main() -> () {
    let ep_addr = ExternalProvider::new(vec![
        InstanceType::new(Vm::new(2, 1024), 10.0),
        InstanceType::new(Vm::new(4, 2048), 20.0),
        InstanceType::new(Vm::new(8, 2048), 30.0),
        InstanceType::new(Vm::new(8, 4096), 40.0),
    ])
    .start();

    let sp = ServiceProvider::new(vec![ep_addr.clone()]);
    let instance_types = sp.request_instance_types(&sp.eps[0]).await;
    println!("RESULT: {:?}", instance_types);

    System::current().stop();
}
