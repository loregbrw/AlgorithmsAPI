use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct MergeSort;

impl MergeSort {
    fn merge(&self, array: &mut Vec<f64>, left: usize, mid: usize, right: usize) {
        let left_array: Vec<f64> = array[left..=mid].to_vec();
        let right_array: Vec<f64> = array[mid + 1..=right].to_vec();

        let mut index = left;
        let (mut index_left, mut index_right) = (0, 0);

        while index_left < left_array.len() && index_right < right_array.len() {
            if left_array[index_left] <= right_array[index_right] {
                array[index] = left_array[index_left];
                index_left += 1;
            } else {
                array[index] = right_array[index_right];
                index_right += 1;
            }

            index += 1;
        }

        while index_left < left_array.len() {
            array[index] = left_array[index_left];
            index_left += 1;
            index += 1;
        }

        while index_right < right_array.len() {
            array[index] = right_array[index_right];
            index_right += 1;
            index += 1;
        }
    }

    fn merge_sort(&self, array: &mut Vec<f64>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;

        self.merge_sort(array, left, mid);
        self.merge_sort(array, mid + 1, right);

        self.merge(array, left, mid, right);
    }
}

impl BaseAlgorithm<Vec<f64>, Vec<f64>> for MergeSort {
    fn name(&self) -> &'static str {
        "Merge Sort"
    }

    fn complexity(&self) -> &'static str {
        "O(n log n)"
    }

    fn run(&self, mut input: Vec<f64>) -> Vec<f64> {
        if input.len() > 1 {
            let size = input.len();
            self.merge_sort(&mut input, 0, size - 1);
        }

        input
    }
}

impl SortAlgorithm for MergeSort {}
