use crate::algorithms::{sort::{bubble_sort::BubbleSort, selection_sort::SelectionSort}, SortAlgorithm};
use std::collections::HashMap;

pub mod bubble_sort;
pub mod selection_sort;

type SortAlgorithmFactory = fn() -> Box<dyn SortAlgorithm>;

pub fn sort_algorithms_endpoints() -> HashMap<&'static str, SortAlgorithmFactory> {
    let mut map: HashMap<&'static str, SortAlgorithmFactory> = HashMap::new();

    map.insert("bubble", || Box::new(BubbleSort));
    map.insert("bubble-sort", || Box::new(BubbleSort));

    map.insert("selection", || Box::new(SelectionSort));
    map.insert("selection-sort", || Box::new(SelectionSort));

    map
}
