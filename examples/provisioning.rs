use actix::prelude::*;

use mist::{
    Vm,
    ExternalProvider,
    ServiceProvider,
};

#[actix_rt::main]
async fn main() -> () {
    let ep_addr = ExternalProvider::new(
        vec![
            Vm::new(2, 1024),
            Vm::new(4, 2048),
            Vm::new(8, 2048),
            Vm::new(8, 4096),
        ]
    ).start();

    let sp = ServiceProvider::new(vec![ep_addr.clone()]);
    let instance_types = sp.request_instance_types(&sp.eps[0]).await;
    println!("RESULT: {:?}", instance_types);

    System::current().stop();
}
