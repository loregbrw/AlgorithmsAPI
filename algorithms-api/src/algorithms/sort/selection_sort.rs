use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct SelectionSort;

impl BaseAlgorithm<Vec<f64>, Vec<f64>> for SelectionSort {
    fn name(&self) -> &'static str {
        "Selection Sort"
    }

    fn complexity(&self) -> &'static str {
        "O(n²)"
    }

    fn run(&self, mut array: Vec<f64>) -> Vec<f64> {
        let n = array.len();

        for i in 0..(n - 1) {
            let mut index_min = i;

            for j in (i + 1)..n {
                if array[j] < array[index_min] {
                    index_min = j;
                }
            }

            array.swap(i, index_min);
        }

        array
    }
}

impl SortAlgorithm for SelectionSort {}
