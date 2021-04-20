use std::time::Instant;

use crate::vdb12::{Application, PublicProvider, ScheduleCost};

pub struct PublicScheduler {
    public_providers: Vec<PublicProvider>,
}

impl PublicScheduler {
    pub fn new(public_providers: Vec<PublicProvider>) -> Self {
        Self { public_providers }
    }

    pub fn cheapest_public_provider(
        &mut self,
        application: &Application,
        now: Instant,
    ) -> Option<&mut PublicProvider> {
        self.public_providers
            .iter_mut()
            .filter_map(|provider| match provider.cost(application, now) {
                ScheduleCost::Impossible => None,
                ScheduleCost::Possible(cost) => Some((provider, cost)),
            })
            .min_by(|(_, left), (_, right)| left.partial_cmp(right).unwrap())
            .map(|(provider, _)| provider)
    }
}
