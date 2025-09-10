use crate::algorithms::{BaseAlgorithm, SortAlgorithm};

pub struct HeapSort;

impl HeapSort {
    fn heapify(&self, array: &mut Vec<f64>, n: usize, i: usize) {
        let mut index_largest = i;
        let (index_left, index_right) = (2 * i + 1, 2 * i + 2);

        if index_left < n && array[index_left] > array[index_largest] {
            index_largest = index_left;
        }

        if index_right < n && array[index_right] > array[index_largest] {
            index_largest = index_right;
        }

        if (index_largest != i) {
            array.swap(i, index_largest);
            self.heapify(array, n, index_largest);
        }
    }
}

impl BaseAlgorithm<Vec<f64>, Vec<f64>> for HeapSort {
    fn name(&self) -> &'static str {
        "Heap Sort"
    }

    fn complexity(&self) -> &'static str {
        "O(n log n)"
    }

    fn run(&self, mut input: Vec<f64>) -> Vec<f64> {
        let size = input.len();

        for i in (0..size / 2).rev() {
            self.heapify(&mut input, size, i);
        }

        for i in (1..size).rev() {
            input.swap(0, i);
            self.heapify(&mut input, i, 0);
        }

        input
    }
}

impl SortAlgorithm for HeapSort {}
