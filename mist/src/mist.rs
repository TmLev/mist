use std::io::Write;
use std::time::{Duration, Instant};

use env_logger;
use log;

use stakker::{actor, ret_nop, Actor, ActorOwn, Stakker};

use crate::algorithms::AlgorithmContext;
use crate::vdb12::{
    Context, Customer, HybridScheduler, ServiceProvider, SortingPolicy, UnfeasiblePolicy,
};
use crate::Algorithm;

pub struct Mist {
    core: Stakker,
    max_steps: Option<usize>,
    current_step: usize,
    algorithm_context: AlgorithmContext,
}

impl Mist {
    pub fn new(algorithm: Algorithm) -> Self {
        // Logging.
        // TODO(TmLev): format simulation time & current step.
        env_logger::builder()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] [{}] {}",
                    record.level(),
                    record.target(),
                    record.args()
                )
            })
            .init();

        // Simulation core.
        let mut core = Stakker::new(Instant::now());
        let stakker = &mut core;

        // Algorithm context.
        let algorithm_context = match algorithm {
            Algorithm::Vdb12 => AlgorithmContext::Vdb12(Context::new(stakker)),
        };

        // Ready.
        Self {
            core,
            max_steps: None,
            current_step: 0,
            algorithm_context,
        }
    }

    // Builder pattern.
    pub fn with_max_steps(mut self, s: usize) -> Self {
        self.max_steps = Some(s);
        self
    }

    pub fn run(&mut self) {
        log::info!("Starting simulation");

        let stakker = &mut self.core;
        let mut now = Instant::now();
        stakker.run(now, false);

        while stakker.not_shutdown() {
            now += stakker.next_wait_max(now, Duration::from_secs(10), false);
            stakker.run(now, false);

            self.current_step += 1;
            let reached_max_steps = match self.max_steps {
                None => false,
                Some(max) => self.current_step > max,
            };
            if reached_max_steps {
                log::info!("Reached maximum number of steps");
                break;
            }
        }

        log::info!("Stopping simulation");
    }
}
