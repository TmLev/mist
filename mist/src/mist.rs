use std::time::{Duration, Instant};

use env_logger;
use log;

use stakker::{actor, ret_nop, Actor, ActorOwn, Stakker};

use crate::vdb12::{Customer, HybridScheduler, ServiceProvider, SortingPolicy, UnfeasiblePolicy};
use crate::Algorithm;

// TODO(TmLev): stubbed with `vdb12` for now.
pub struct Mist {
    core: Stakker,
    service_provider: ActorOwn<ServiceProvider>,
    customers: Vec<ActorOwn<Customer>>,
}

impl Mist {
    pub fn new(algorithm: Algorithm) -> Self {
        // Logging.
        env_logger::init();

        // Simulation core.
        let mut core = Stakker::new(Instant::now());
        let stakker = &mut core;

        // Service provider.
        let hybrid_scheduler = HybridScheduler::new(
            vec![],
            SortingPolicy::FirstComeFirstServed,
            UnfeasiblePolicy::UnfeasibleToPublic,
        );
        let service_provider = actor!(stakker, ServiceProvider::init(hybrid_scheduler), ret_nop!());

        // Customers.
        let customers = vec![actor!(
            stakker,
            Customer::init(service_provider.clone()),
            ret_nop!()
        )];

        // Ready.
        Self {
            core,
            service_provider,
            customers,
        }
    }

    pub fn run(&mut self) {
        log::info!("Starting simulation");

        let stakker = &mut self.core;
        let mut now = Instant::now();

        let mut count = 0;

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
}
