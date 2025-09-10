use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct MergeSort;

impl MergeSort {
    fn merge(&self, array: &mut Vec<f64>, left: usize, mid: usize, right: usize) {
        let l: Vec<f64> = array[left..=mid].to_vec();
        let r: Vec<f64> = array[mid + 1..=right].to_vec();

        let mut i = 0;
        let mut j = 0;
        let mut k = left;

        while i < l.len() && j < r.len() {
            if l[i] <= r[j] {
                array[k] = l[i];
                i += 1;
            } else {
                array[k] = r[j];
                j += 1;
            }
            k += 1;
        }

        while i < l.len() {
            array[k] = l[i];
            i += 1;
            k += 1;
        }

        while j < r.len() {
            array[k] = r[j];
            j += 1;
            k += 1;
        }
    }

    fn merge_sort(&self, array: &mut Vec<f64>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;

        Self::merge_sort(&self, array, left, mid);
        Self::merge_sort(&self, array, mid + 1, right);

        Self::merge(&self, array, left, mid, right);
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
