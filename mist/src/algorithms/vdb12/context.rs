use stakker::{actor, ret_nop, ActorOwn, Stakker};

use crate::vdb12::{Customer, HybridScheduler, ServiceProvider, SortingPolicy, UnfeasiblePolicy};

pub struct Context {
    service_provider: ActorOwn<ServiceProvider>,
    customers: Vec<ActorOwn<Customer>>,
}

impl Context {
    pub fn new(stakker: &mut Stakker) -> Self {
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

        Self {
            service_provider,
            customers,
        }
    }
}
