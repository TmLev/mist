use std::path::Path;

fn main() {
    let simulations_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&simulations_dir).parent().unwrap().join("data");
    println!("{:?}", data_dir);
}
