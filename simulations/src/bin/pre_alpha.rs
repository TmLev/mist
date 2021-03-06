use std::time::Instant;

use mist::{vdb12, Mist};

fn main() {
    // Enable debug logging.
    std::env::set_var("RUST_LOG", "info");

    // Create simulator instance.
    let mut mist = Mist::new(Instant::now());

    // Set maximum number of steps.
    mist.with_max_steps(500);

    // Choose algorithm and access actor model core for creating context.
    let ctx = vdb12::Context::new(mist.actor_model_core());

    // Provide algorithm context.
    mist.with_algorithm_context(ctx);

    // Run simulation
    mist.run();
}
