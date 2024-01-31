//! <https://www.codewars.com/kata/59590976838112bfea0000fa/train/rust>

pub fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    (0..n)
        .map(|i| values.iter().skip(i).step_by(n).sum::<u32>())
        .collect()
}
