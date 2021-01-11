use crate::vm::Vm;

#[derive(Debug)]
pub struct ExternalProvider {
    instance_types: Vec<Vm>,
}

impl ExternalProvider {
    pub fn new(instance_types: Vec<Vm>) -> ExternalProvider {
        ExternalProvider { instance_types }
    }

    pub fn instance_types(&self) -> &[Vm] {
        &self.instance_types
    }
}

