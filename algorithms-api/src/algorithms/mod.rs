pub mod sort;

pub trait SortAlgorithm {
    fn sort<T: PartialOrd>(array: &mut [T]) -> &mut [T];
}
