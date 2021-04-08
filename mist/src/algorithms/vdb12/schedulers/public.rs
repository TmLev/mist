use crate::vdb12::{Application, PublicProvider, ScheduleCost};

pub struct PublicScheduler {
    public_providers: Vec<PublicProvider>,
}

impl PublicScheduler {
    pub fn new(public_providers: Vec<PublicProvider>) -> Self {
        Self { public_providers }
    }

    pub fn cheapest_public_provider(&self, application: &Application) -> &PublicProvider {
        let possible_providers_with_costs: Vec<_> = self
            .public_providers
            .iter()
            .filter_map(|provider| match provider.cost(application) {
                ScheduleCost::Impossible => None,
                ScheduleCost::Possible(cost) => Some((provider, cost)),
            })
            .collect();

        if possible_providers_with_costs.len() == 0 {
            // Public providers *should* be able to execute any application in theory.
            // TODO(TmLev): but only in theory.
            panic!("Public providers can not execute application");
        }

        possible_providers_with_costs
            .iter()
            .min_by(|left, right| left.1.partial_cmp(&right.1).unwrap())
            .unwrap()
            .0
    }
}
