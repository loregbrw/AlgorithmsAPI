use crate::algorithms::SortAlgorithm;

pub struct BubbleSort;

impl SortAlgorithm for BubbleSort {
    fn sort<T: PartialOrd>(array: &mut [T]) -> &mut [T] {
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
