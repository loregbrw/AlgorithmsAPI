// use std::time::Instant;

// use crate::{algorithms::BaseAlgorithm, models::responses::Response};

// pub struct Executor;

// impl Executor {
//     pub fn execute<I, O>(algorithm: Box<dyn BaseAlgorithm<I, O>>, input: I) -> Response<O> {

//         let start = Instant::now();

//         let result = algorithm.run(input);

//         let duration = start.elapsed();

//         let stats = Stats {
//             time_ms: duration.as_millis() as u64,
//             // aqui depois dá pra adicionar consumo de memória
//         };

//         Response {
//             algorithm: AlgorithmResult {
//                 result,
//                 name: algorithm.name(),
//                 complexity: algorithm.complexity(),
//             },
//             stats,
//         }
//     }
// }
