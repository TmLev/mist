#[derive(Debug, Clone)]
pub struct Vm {
    cpu: u32,
    ram: u32,
}

impl Vm {
    pub fn new(cpu: u32, ram: u32) -> Self {
        Self { cpu, ram }
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self { cpu: 4, ram: 2048 }
    }
}

pub type Cost = f64;

#[derive(Debug, Clone)]
pub struct InstanceType {
    vm: Vm,
    cost: Cost,
}

impl InstanceType {
    pub fn new(vm: Vm, cost: Cost) -> Self {
        Self { vm, cost }
    }
}

pub type InstanceTypes = Vec<InstanceType>;
