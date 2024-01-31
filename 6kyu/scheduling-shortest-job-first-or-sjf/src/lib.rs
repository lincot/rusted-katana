//! <https://www.codewars.com/kata/550cc572b9e7b563be00054f/train/rust>

pub fn sjf(jobs: &[usize], index: usize) -> usize {
    let pivot = jobs[index];
    pivot
        + jobs[..index].iter().filter(|&&x| x <= pivot).sum::<usize>()
        + jobs
            .get(index + 1..)
            .map_or(0, |s| s.iter().filter(|&&x| x < pivot).sum::<usize>())
}
