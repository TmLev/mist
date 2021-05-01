use std::io::Write;
use std::time::{Duration, Instant};

use env_logger;
use log;

use stakker::Stakker;

/// The heart of the simulator.
pub struct Mist<AlgorithmContext> {
    /// Actor model core.
    amc: Stakker,
    /// Current instant of the simulation.
    now: Instant,
    /// Current step of the simulation.
    current_step: usize,
    /// Optional limit on the simulation steps.
    max_steps: Option<usize>,
    /// Everything algorithm needs.
    algorithm_context: Option<AlgorithmContext>,
}

impl<AlgorithmContext> Mist<AlgorithmContext> {
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
            amc: Stakker::new(simulation_start_time),
            now: simulation_start_time,
            current_step: 0,
            max_steps: None,
            algorithm_context: None,
        }
    }

    pub fn actor_model_core(&mut self) -> &mut Stakker {
        &mut self.amc
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
        while self.runnable() {
            self.step();
        }
        log::info!("Stopping simulation");
    }

    fn runnable(&mut self) -> bool {
        self.amc.not_shutdown() && !self.reached_max_steps()
    }

    fn step(&mut self) {
        if self.current_step > 0 {
            self.advance_time();
        }
        self.amc.run(self.now, false);
        self.current_step += 1;
    }

    fn advance_time(&mut self) {
        self.now += self.amc.next_wait_max(
            self.now,
            Duration::from_secs(10), // FW(TmLev): customize.
            false,
        );
    }

    fn reached_max_steps(&self) -> bool {
        match self.max_steps {
            None => false,
            Some(max) => {
                log::info!("Reached maximum number of steps");
                self.current_step > max
            }
        }
    }

    /// Get algorithm context back.
    pub fn get_algorithm_context(&mut self) -> Option<AlgorithmContext> {
        self.algorithm_context.take()
    }
}
