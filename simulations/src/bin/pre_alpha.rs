use std::time::Instant;

use mist::vdb12::Context;
use mist::{AlgorithmContext, Mist};

fn main() {
    // Enable debug logging.
    std::env::set_var("RUST_LOG", "info");

    // Create simulator instance.
    let mut mist = Mist::new(Instant::now());

    // Set maximum number of steps.
    mist.with_max_steps(500);

    // Choose algorithm and access Mist core for creating context.
    let ctx = AlgorithmContext::Vdb12(Context::new(mist.core()));

    // Provide algorithm context.
    mist.with_algorithm_context(ctx);

    // Run simulation
    mist.run();
}
