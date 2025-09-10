use crate::{
    algorithms::BaseAlgorithm,
    models::{responses::AlgorithmResponse, stats::Stats},
};
use peak_alloc::PeakAlloc;
use std::time::Instant;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

pub struct Executor;

impl Executor {
    pub fn execute<I, O>(
        algorithm: Box<dyn BaseAlgorithm<I, O>>,
        input: I,
    ) -> AlgorithmResponse<O> {
        PEAK_ALLOC.reset_peak_usage();

        let start = Instant::now();

        let result = algorithm.run(input);

        let duration = start.elapsed();

        let peak_memory_bytes = PEAK_ALLOC.peak_usage();

        AlgorithmResponse {
            result,
            name: algorithm.name(),
            complexity: algorithm.complexity(),
            stats: Stats {
                execution_time_us: duration.as_micros(),
                peak_memory_bytes,
            },
        }
    }
}
