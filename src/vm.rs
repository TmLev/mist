#[derive(Debug)]
pub struct Vm {
    cpu: u32,
    ram: u32,
}

impl Vm {
    pub fn new(cpu: u32, ram: u32) -> Vm {
        Vm {
            cpu,
            ram,
        }
    }
}

impl Default for Vm {
    fn default() -> Vm {
        Vm {
            cpu: 4,
            ram: 2048,
        }
    }
}
