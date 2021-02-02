use mist::rusty_money::{iso::USD, Money};
use mist::{ExternalProvider, InstanceType, ServiceProvider, Vm};

async fn main() -> () {
    let ep_addr = ExternalProvider::new(vec![
        InstanceType::new(Vm::new(2, 1024), Money::from_major(10, USD)),
        InstanceType::new(Vm::new(4, 2048), Money::from_major(20, USD)),
        InstanceType::new(Vm::new(8, 2048), Money::from_major(30, USD)),
        InstanceType::new(Vm::new(8, 4096), Money::from_major(40, USD)),
    ])
    .start();

    let sp = ServiceProvider::new(vec![ep_addr.clone()]);
    let instance_types = sp.request_instance_types(&sp.eps[0]).await;
    println!("RESULT: {:?}", instance_types);

    System::current().stop();
}
