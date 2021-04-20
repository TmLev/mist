use std::path::PathBuf;

pub(crate) fn data_dir() -> PathBuf {
    let simulations_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    PathBuf::from(&simulations_dir)
        .parent()
        .unwrap()
        .join("data")
}
