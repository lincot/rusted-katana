//! <https://www.codewars.com/kata/5711fc7c159cde6ac70003e2/train/rust>

use either::Either;

pub fn find_zero_sum_groups(arr: &[i32], n: u32) -> Either<&'static str, Vec<Vec<i32>>> {
    if arr.is_empty() {
        return Either::Left("No elements to combine");
    }

    let mut arr = arr.to_vec();
    arr.sort_unstable();
    arr.dedup();
    let n = n as usize;
    if n > arr.len() {
        return Either::Left("No combinations");
    }

    let mut res = Vec::new();
    let mut indices: Box<[_]> = (0..n).collect();
    let mut current_sum: i32 = indices.iter().map(|&i| arr[i]).sum();

    while indices[0] <= arr.len() - n {
        if current_sum == 0 {
            res.push(indices.iter().map(|&i| arr[i]).collect());
        }

        let mut found = false;
        for i in (0..n).rev() {
            if indices[i] < arr.len() - (n - i) {
                indices[i] += 1;
                for j in (i + 1)..n {
                    indices[j] = indices[j - 1] + 1;
                }
                current_sum = indices
                    .iter()
                    .map(|&index| unsafe { arr.get_unchecked(index) })
                    .sum();
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
    }

    if res.is_empty() {
        Either::Left("No combinations")
    } else {
        Either::Right(res)
    }
}
