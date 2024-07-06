//! <https://www.codewars.com/kata/52fba2a9adcd10b34300094c/train/rust>

use core::iter::repeat_with;

pub fn transpose(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    if matrix.is_empty() {
        return Vec::new();
    }
    let row_len = matrix[0].len();
    assert!(matrix[1..].iter().all(|row| row.len() == row_len));

    unsafe {
        if matrix.len() < 1024 {
            transpose_column_first(matrix)
        } else {
            transpose_row_first(matrix)
        }
    }
}

unsafe fn transpose_row_first(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut res: Vec<_> = repeat_with(|| {
        let mut v = Vec::with_capacity(matrix.len());
        v.set_len(matrix.len());
        v
    })
    .take(matrix[0].len())
    .collect();
    for (j, row) in matrix.iter().enumerate() {
        for i in 0..matrix[0].len() {
            *res.get_unchecked_mut(i).get_unchecked_mut(j) = *row.get_unchecked(i);
        }
    }
    res
}

unsafe fn transpose_column_first(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    (0..matrix[0].len())
        .map(|j| {
            (0..matrix.len())
                .map(|i| *unsafe { matrix.get_unchecked(i).get_unchecked(j) })
                .collect()
        })
        .collect()
}

#[cfg(test)]
#[cfg(miri)]
mod miri_tests {
    use super::*;
    use core::array;

    #[test]
    fn miri_test() {
        let matrix: [_; 10] = array::from_fn(|_| (0..=20).collect::<Vec<_>>());
        unsafe {
            transpose(&matrix);
            transpose_row_first(&matrix);
            transpose_column_first(&matrix);
        }
    }
}
