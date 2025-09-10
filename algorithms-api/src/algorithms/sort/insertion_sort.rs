use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct InsertionSort;

impl BaseAlgorithm<Vec<f64>, Vec<f64>> for InsertionSort {
    fn name(&self) -> &'static str {
        "Insertion Sort"
    }

    fn complexity(&self) -> &'static str {
        "O(n²)"
    }

    fn run(&self, mut input: Vec<f64>) -> Vec<f64> {
        let size = input.len();

        for i in 1..size {
            let key = input[i];
            let mut j = i;

            while j > 0 && input[j - 1] > key {
                input[j] = input[j - 1];
                j -= 1;
            }

            input[j] = key;
        }

        input
    }
}

impl SortAlgorithm for InsertionSort {}
