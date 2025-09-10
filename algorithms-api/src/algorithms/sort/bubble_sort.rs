use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct BubbleSort;

impl BaseAlgorithm<Vec<f64>, Vec<f64>> for BubbleSort {
    fn name(&self) -> &'static str {
        "Bubble Sort"
    }

    fn complexity(&self) -> &'static str {
        "O(n²)"
    }

    fn run(&self, mut input: Vec<f64>) -> Vec<f64> {
        let size = input.len();

        for i in 0..size {
            let mut swapped = false;

            for j in 0..(size - i - 1) {
                if input[j] > input[j + 1] {
                    input.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }

        input
    }
}

impl SortAlgorithm for BubbleSort {}