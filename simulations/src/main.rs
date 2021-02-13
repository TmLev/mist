use std::time::{Duration, Instant};

use stakker::{actor, ret_nop, Stakker};

use mist::rusty_money::{iso::USD, Money};
use mist::{ExternalProvider, InstanceType, ServiceProvider, Vm};

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
