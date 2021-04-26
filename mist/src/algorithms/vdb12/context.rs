use std::{cell::RefCell, rc::Rc};

use stakker::{actor, ret_nop, ActorOwn, Stakker};

use crate::utils::navigation;
use crate::vdb12::{
    Customer, HybridScheduler, Metrics, PrivateScheduler, PublicProvider, PublicScheduler,
    ServiceProvider, SortingPolicy, UnfeasiblePolicy,
};

#[allow(dead_code)]
pub struct Context {
    service_provider: ActorOwn<ServiceProvider>,
    customers: Vec<ActorOwn<Customer>>,
    metrics: Rc<RefCell<Metrics>>,
}

impl Context {
    pub fn new(core: &mut Stakker) -> Self {
        // Navigate directory with instance types.
        let instance_types_dir = navigation::data_dir().join("vdb12/instance-types");

        // Public providers & scheduler.
        let public_providers = vec![
            PublicProvider::from_file(instance_types_dir.clone().join("go-grid.json")).unwrap(),
            PublicProvider::from_file(instance_types_dir.clone().join("amazon-ec2.json")).unwrap(),
        ];
        let public_scheduler = PublicScheduler::new(public_providers);

        // Private scheduler.
        let private_scheduler =
            PrivateScheduler::from_file(instance_types_dir.join("private.json")).unwrap();

        // Galactus
        let galactus = Rc::new(RefCell::new(Metrics::new()));

        // Hybrid scheduler & service provider.
        let hybrid_scheduler = HybridScheduler::new(
            core.now(),
            SortingPolicy::FirstComeFirstServed,
            UnfeasiblePolicy::UnfeasibleToPublic,
            private_scheduler,
            public_scheduler,
            galactus.clone(),
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
            metrics: galactus,
        }
    }

    pub fn metrics(&self) -> Metrics {
        self.metrics.borrow().clone()
    }
}
