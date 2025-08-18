use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct BubbleSort;

impl BaseAlgorithm<Vec<f64>, Vec<f64>> for BubbleSort {
    fn name(&self) -> &'static str {
        "Bubble Sort"
    }

    fn complexity(&self) -> &'static str {
        "O(n²)"
    }

    fn run(&self, mut array: Vec<f64>) -> Vec<f64> {
        let size = array.len();

        for i in 0..size {
            let mut swapped = false;

            for j in 0..(size - i - 1) {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }

        array
    }
}

impl SortAlgorithm for BubbleSort {}