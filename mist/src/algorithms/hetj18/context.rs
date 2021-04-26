use std::{cell::RefCell, rc::Rc};

use stakker::{actor, ret_nop, ActorOwn, Stakker};

use crate::hetj18::{Metrics, PrivatePool, PublicPool, ServiceProvider, User};

pub struct Context {
    service_provider: ActorOwn<ServiceProvider>,
    users: Vec<ActorOwn<User>>,
    metrics: Rc<RefCell<Metrics>>,
}

impl Context {
    pub fn new(core: &mut Stakker) -> Self {
        let metrics = Rc::new(RefCell::new(Metrics::new()));

        // Private pool.
        let private_pool = PrivatePool::new(vec![]);

        // Public cloud.
        let public_cloud = vec![PublicPool {}];

        // Service provider.
        let service_provider = actor!(
            core,
            ServiceProvider::init(private_pool, public_cloud),
            ret_nop!()
        );

        let users = vec![actor!(core, User::init(), ret_nop!())];

        Self {
            service_provider,
            users,
            metrics,
        }
    }
}
