use mist::{
    Vm,
    ExternalProvider,
    ServiceProvider,
};

fn main() {
    let vm = mist::Vm::default();
    println!("{:?}", vm);

    let external_provider_1 = ExternalProvider::new(
        vec![
            Vm::new(2, 1024),
            Vm::new(4, 2048),
            Vm::new(8, 2048),
            Vm::new(8, 4096),
        ]
    );
    println!("{:?}", external_provider_1);
    println!("{:?}", external_provider_1.instance_types());

    let external_provider_2 = ExternalProvider::new(
        vec![
            Vm::new(2, 4096),
            Vm::new(8, 8192),
        ]
    );

    let service_provider = ServiceProvider::new(
        vec![
            &external_provider_1,
            &external_provider_2,
        ]
    );
    println!("{:?}", service_provider);
    println!("{:?}", service_provider.public_cloud());
}
