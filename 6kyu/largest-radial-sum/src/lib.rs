//! <https://www.codewars.com/kata/64edf7ab2b610b16c2067579/train/rust>

pub fn largest_radial_sum(arr: &[i32], d: u32) -> i32 {
    let mut sums = vec![0; arr.len() / d as usize];
    arr.chunks_exact(sums.len())
        .for_each(|chunk| sums.iter_mut().zip(chunk).for_each(|(s, x)| *s += x));
    sums.into_iter().max().unwrap()
}
