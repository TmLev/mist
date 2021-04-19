use stakker::{actor, ret_nop, ActorOwn, Stakker};

use crate::vdb12::{
    Customer, HybridScheduler, InstanceType, PublicProvider, PublicScheduler, ServiceProvider,
    SortingPolicy, UnfeasiblePolicy, Vm,
};

pub struct Context {
    service_provider: ActorOwn<ServiceProvider>,
    customers: Vec<ActorOwn<Customer>>,
}

impl Context {
    pub fn new(core: &mut Stakker) -> Self {
        // Service provider.
        let public_providers = vec![PublicProvider::new(vec![InstanceType {
            vm: Vm { cpu: 16, mem: 16 },
            price: 2.0,
            billing_interval: 3600.0,
        }])];
        let public_scheduler = PublicScheduler::new(public_providers);
        let hybrid_scheduler = HybridScheduler::new(
            core.now(),
            vec![],
            SortingPolicy::FirstComeFirstServed,
            UnfeasiblePolicy::UnfeasibleToPublic,
            public_scheduler,
        );
        let service_provider = actor!(core, ServiceProvider::init(hybrid_scheduler), ret_nop!());

        // Customers.
        let customers = vec![actor!(
            core,
            Customer::init(service_provider.clone()),
            ret_nop!()
        )];

        Self {
            service_provider,
            customers,
        }
    }
}
