//! <https://www.codewars.com/kata/5a905c2157c562994900009d/train/rust>

pub fn product_array(arr: &[u64]) -> Vec<u64> {
    let prod: u64 = arr.iter().product();
    arr.iter().map(|x| prod / x).collect()
}
