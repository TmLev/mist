use crate::ExternalProvider;

#[derive(Debug)]
pub struct ServiceProvider<'ep> {
     public_cloud: Vec<&'ep ExternalProvider>,
}

impl<'ep> ServiceProvider<'ep> {
     pub fn new(public_cloud: Vec<&'ep ExternalProvider>) -> ServiceProvider {
          ServiceProvider { public_cloud }
     }

     pub fn public_cloud(&self) -> &[&'ep ExternalProvider] {
          &self.public_cloud
     }
}
