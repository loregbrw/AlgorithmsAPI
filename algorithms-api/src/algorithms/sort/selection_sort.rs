use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct SelectionSort;

impl BaseAlgorithm<Vec<f64>, Vec<f64>> for SelectionSort {
    fn name(&self) -> &'static str {
        "Selection Sort"
    }

    fn complexity(&self) -> &'static str {
        "O(n²)"
    }

    fn run(&self, mut input: Vec<f64>) -> Vec<f64> {
        let size = input.len();

        for i in 0..(size - 1) {
            let mut index_min = i;

            for j in (i + 1)..size {
                if input[j] < input[index_min] {
                    index_min = j;
                }
            }

            input.swap(i, index_min);
        }

        input
    }
}

impl SortAlgorithm for SelectionSort {}
