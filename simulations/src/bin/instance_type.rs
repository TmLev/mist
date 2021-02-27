use mist::vdb12::{InstanceType, Vm};

fn main() {
    let vm = Vm { cpu: 1, mem: 1700 };
    let it = InstanceType {
        vm,
        price: 0.0085,
        billing_period: 60.0 * 60.0,
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