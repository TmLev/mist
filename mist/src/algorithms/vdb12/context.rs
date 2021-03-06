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
    pub fn new(amc: &mut Stakker) -> Self {
        // Navigate directory with instance types.
        let instance_types_dir = navigation::data_dir().join("vdb12/instance-types");

        // Public providers & scheduler.
        let public_providers = vec![
            PublicProvider::from_file(instance_types_dir.join("go-grid.json")).unwrap(),
            PublicProvider::from_file(instance_types_dir.join("amazon-ec2.json")).unwrap(),
        ];
        let public_scheduler = PublicScheduler::new(public_providers);

        // Private scheduler.
        let private_scheduler =
            PrivateScheduler::from_file(instance_types_dir.join("private.json")).unwrap();

        // Metrics.
        let metrics = Rc::new(RefCell::new(Metrics::new()));

        // Hybrid scheduler & service provider.
        let hybrid_scheduler = HybridScheduler::new(
            amc.now(),
            SortingPolicy::FirstComeFirstServed,
            UnfeasiblePolicy::UnfeasibleToPublic,
            private_scheduler,
            public_scheduler,
            metrics.clone(),
        );
        let service_provider = actor!(amc, ServiceProvider::init(hybrid_scheduler), ret_nop!());

        // Customers.
        let customers = vec![actor!(
            amc,
            Customer::init(service_provider.clone()),
            ret_nop!()
        )];

        Self {
            service_provider,
            customers,
            metrics,
        }
    }

    pub fn metrics(&self) -> Metrics {
        self.metrics.borrow().clone()
    }
}
