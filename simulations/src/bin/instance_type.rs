use mist::vdb12::{InstanceType, VirtualMachine};

fn main() {
    let vm = VirtualMachine { cpu: 1, mem: 1700 };
    let it = InstanceType {
        characteristics: vm,
        price: 0.0085,
        billing_interval: 60.0 * 60.0,
    };
    println!("{}", it.to_json().unwrap());

    println!(
        "{:?}",
        InstanceType::from_json(
            r#"
{
  "vm": {
    "cpu": 1,
    "mem": 1700
  },
  "price": 0.0085,
  "billing_period": 3600.0
}"#
        )
        .unwrap()
    );
}
