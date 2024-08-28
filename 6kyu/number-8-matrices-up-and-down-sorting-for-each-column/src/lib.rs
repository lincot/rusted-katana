//! <https://www.codewars.com/kata/590b8d5cee471472f40000aa/train/rust>

use unchecked_std::prelude::*;

pub fn up_down_col_sort(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = matrix[0].len();

    let mut all_items = Vec::with_capacity(n * matrix.len());
    for row in matrix {
        assert!(row.len() == n);
        unsafe { all_items.extend_from_slice_unchecked(row) };
    }
    vqsort_rs::sort(&mut all_items);

    let mut res: Vec<_> = (0..matrix.len())
        .map(|_| {
            let mut row = Vec::with_capacity(n);
            unsafe { row.set_len(n) };
            row
        })
        .collect();
    for i in 0..matrix.len() {
        let mut bias = 0;
        for j in 0..n {
            let all_items_i = if j % 2 == 0 {
                i + bias
            } else {
                i + bias - matrix.len() + 2 * (matrix.len() - i) - 1
            };
            bias += matrix.len();
            unsafe {
                *res.get_unchecked_mut(i).get_unchecked_mut(j) =
                    *all_items.get_unchecked(all_items_i);
            }
        }
    }
    res
}
