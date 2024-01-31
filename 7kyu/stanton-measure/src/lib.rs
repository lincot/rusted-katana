//! <https://www.codewars.com/kata/59a1cdde9f922b83ee00003b/train/rust>

pub fn stanton_measure(arr: &[u32]) -> u32 {
    let n = arr.iter().filter(|&&x| x == 1).count() as _;
    arr.iter().filter(|&&x| x == n).count() as _
}
