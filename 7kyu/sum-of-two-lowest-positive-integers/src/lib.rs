//! <https://www.codewars.com/kata/558fc85d8fd1938afb000014/train/rust>

pub fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let mut min0 = u32::MAX;
    let mut min1 = u32::MAX;
    for &n in numbers {
        if n <= min0 {
            min1 = min0;
            min0 = n;
        } else if n <= min1 {
            min1 = n;
        }
    }
    min0 + min1
}
