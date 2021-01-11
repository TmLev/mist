use mist::{
    Vm,
    ExternalProvider,
};

fn main() {
    let vm = mist::Vm::default();
    println!("{:?}", vm);

    let provider = ExternalProvider::new(
        vec![
            Vm::new(2, 1024),
            Vm::new(4, 2048),
            Vm::new(8, 2048),
            Vm::new(8, 4096),
        ]
    );
    println!("{:?}", provider);
    println!("{:?}", provider.instance_types());
}
