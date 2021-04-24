use std::io::Write;
use std::time::{Duration, Instant};

use env_logger;
use log;

use stakker::Stakker;

use crate::AlgorithmContext;

/// The heart of the simulator.
pub struct Mist {
    /// Main event loop.
    core: Stakker,
    /// Current step of the simulation.
    current_step: usize,
    /// Optional limit on the simulation steps.
    max_steps: Option<usize>,
    /// Everything algorithm needs.
    algorithm_context: Option<AlgorithmContext>,
}

impl Mist {
    pub fn new(simulation_start_time: Instant) -> Self {
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

        Self {
            core: Stakker::new(simulation_start_time),
            current_step: 0,
            max_steps: None,
            algorithm_context: None,
        }
    }

    pub fn core(&mut self) -> &mut Stakker {
        &mut self.core
    }

    pub fn with_max_steps(&mut self, s: usize) -> &mut Self {
        self.max_steps = Some(s);
        self
    }

    pub fn with_algorithm_context(&mut self, ctx: AlgorithmContext) -> &mut Self {
        self.algorithm_context = Some(ctx);
        self
    }

    pub fn run(&mut self) {
        log::info!("Starting simulation");

        let stakker = &mut self.core;
        let mut now = Instant::now();
        stakker.run(now, false);

        while stakker.not_shutdown() {
            now += stakker.next_wait_max(
                now,
                Duration::from_secs(10), // TODO(TmLev): customize.
                false,
            );
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
