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
