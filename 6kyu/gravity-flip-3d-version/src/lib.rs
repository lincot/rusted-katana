//! <https://www.codewars.com/kata/5f849ab530b05d00145b9495/train/rust>

pub fn flip(direction: char, mut matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return matrix;
    }

    match direction {
        'R' => {
            for row in &mut matrix {
                vqsort_rs::sort(row);
            }
        }
        'L' => {
            for row in &mut matrix {
                vqsort_rs::sort_descending(row);
            }
        }
        _ => {
            let len = matrix.len() * matrix[0].len();
            let mut transposed = Vec::with_capacity(len);
            for (i, row) in matrix.iter().enumerate() {
                assert!(row.len() == matrix[0].len());
                let mut t_i = i;
                for &x in row {
                    unsafe {
                        transposed
                            .spare_capacity_mut()
                            .get_unchecked_mut(t_i)
                            .write(x);
                    }
                    t_i += matrix.len();
                }
            }
            unsafe { transposed.set_len(len) };

            let mut start = 0;
            if direction == 'D' {
                for _ in 0..matrix[0].len() {
                    vqsort_rs::sort(unsafe {
                        transposed.get_unchecked_mut(start..start + matrix.len())
                    });
                    start += matrix.len();
                }
            } else {
                for _ in 0..matrix[0].len() {
                    vqsort_rs::sort_descending(unsafe {
                        transposed.get_unchecked_mut(start..start + matrix.len())
                    });
                    start += matrix.len();
                }
            }
            let mut t_i = 0;
            for i in 0..matrix[0].len() {
                for j in 0..matrix.len() {
                    unsafe {
                        *matrix.get_unchecked_mut(j).get_unchecked_mut(i) =
                            *transposed.get_unchecked(t_i);
                    }
                    t_i += 1;
                }
            }
        }
    }
    matrix
}
