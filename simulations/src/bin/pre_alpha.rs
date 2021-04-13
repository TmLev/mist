use mist::{Algorithm, Mist};

fn main() {
    // Enable debug logging
    std::env::set_var("RUST_LOG", "debug");

    // Create simulator instance
    let mut mist = Mist::new(Algorithm::Vdb12);

    // Run simulation
    mist.run();
}
