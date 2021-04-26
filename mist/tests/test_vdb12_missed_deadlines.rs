use std::time::Instant;

use mist::{vdb12, Mist};

#[test]
fn impossible_applications_miss_deadline() {
    std::env::set_var("RUST_LOG", "debug");

    let mut mist = Mist::new(Instant::now());

    let ctx = vdb12::Context::new(mist.core());

    mist.with_max_steps(100).with_algorithm_context(ctx).run();

    let ctx = mist.get_algorithm_context().unwrap();
    println!("{:?}", ctx.metrics());
}
