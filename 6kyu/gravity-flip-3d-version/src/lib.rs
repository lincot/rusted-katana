//! <https://www.codewars.com/kata/5f849ab530b05d00145b9495/train/rust>

use vqsort::VqSort;

pub fn flip(direction: char, mut matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return matrix;
    }

    match direction {
        'R' => {
            for row in &mut matrix {
                VqSort::sort(row);
            }
        }
        'L' => {
            for row in &mut matrix {
                VqSort::sort_descending(row);
            }
        }
        _ => {
            assert!(matrix.iter().all(|row| row.len() == matrix[0].len()));
            let mut transposed = vec![
                {
                    let mut res = Vec::<u32>::with_capacity(matrix.len());
                    unsafe { res.set_len(matrix.len()) };
                    res
                };
                matrix[0].len()
            ];
            for (i, row) in matrix.iter().enumerate() {
                for (j, &x) in row.iter().enumerate() {
                    unsafe { *transposed.get_unchecked_mut(j).get_unchecked_mut(i) = x };
                }
            }
            if direction == 'D' {
                for row in &mut transposed {
                    VqSort::sort(row);
                }
            } else {
                for row in &mut transposed {
                    VqSort::sort_descending(row);
                }
            }
            for (i, row) in transposed.iter().enumerate() {
                for (j, &x) in row.iter().enumerate() {
                    unsafe { *matrix.get_unchecked_mut(j).get_unchecked_mut(i) = x };
                }
            }
        }
    }
    matrix
}
