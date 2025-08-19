use std::time::Instant;

use crate::{
    algorithms::BaseAlgorithm,
    models::{responses::AlgorithmResponse, stats::Stats},
};

pub struct Executor;

impl Executor {
    pub fn execute<I, O>(
        algorithm: Box<dyn BaseAlgorithm<I, O>>,
        input: I,
    ) -> AlgorithmResponse<O> {
        
        let start = Instant::now();

        let result = algorithm.run(input);

        let duration = start.elapsed();

        AlgorithmResponse {
            result,
            name: algorithm.name(),
            complexity: algorithm.complexity(),
            stats: Stats {
                execution_time_us: duration.as_micros(),
                memory_used_bytes: 0 as usize,
            },
        }
    }
}
