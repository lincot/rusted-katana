//! <https://www.codewars.com/kata/6066ae080168ff0032c4107a/train/rust>

pub fn max_sum_between_two_negatives(nums: &[i32]) -> Option<i32> {
    let mut sum = 0;
    let mut res = -1;
    let mut inside = false;
    for &num in nums {
        if num < 0 {
            if inside && sum > res {
                res = sum;
            }
            sum = 0;
            inside = true;
        } else if inside {
            sum += num;
        }
    }
    if res >= 0 {
        Some(res)
    } else {
        None
    }
}
