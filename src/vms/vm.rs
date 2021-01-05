#[derive(Debug)]
struct Vm {
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
