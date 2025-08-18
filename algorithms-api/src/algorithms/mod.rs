pub mod sort;

pub trait BaseAlgorithm<I, O> {
    fn name(&self) -> &'static str;
    fn complexity(&self) -> &'static str;
    fn run(&self, input: I) -> O;
}

pub trait SortAlgorithm: BaseAlgorithm<Vec<f64>, Vec<f64>> {}