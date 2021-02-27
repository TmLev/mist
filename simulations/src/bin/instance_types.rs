use std::fs::File;
use std::io::Read;
use std::path::Path;

use mist::serde_json;
use mist::vdb12::InstanceType;

fn main() {
    let simulations_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&simulations_dir).parent().unwrap().join("data");
    let mut file = File::open(data_dir.join("vdb12/instance-types/amazon-ec2.json")).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Vec<InstanceType> = serde_json::from_str(&data).unwrap();
    println!("{:?}", json);
}
