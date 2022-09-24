//! <https://www.codewars.com/kata/550cc572b9e7b563be00054f/train/rust>

#![no_std]

pub fn sjf(jobs: &[usize], index: usize) -> usize {
    let pivot = jobs[index];
    pivot
        + jobs[..index].iter().filter(|&&x| x <= pivot).sum::<usize>()
        + jobs[index + 1..]
            .iter()
            .filter(|&&x| x < pivot)
            .sum::<usize>()
}
