//! <https://www.codewars.com/kata/5d0365accfd09600130a00c9/train/rust>

pub fn solve(vecs: &[Vec<i32>]) -> i32 {
    vecs.iter()
        .map(|vec| min_max(vec))
        .fold((1, 1), |(gmin, gmax), (lmin, lmax)| {
            min_max(&[gmin * lmin, gmin * lmax, gmax * lmin, gmax * lmax])
        })
        .1
}

fn min_max(arr: &[i32]) -> (i32, i32) {
    arr.iter().fold((i32::MAX, i32::MIN), |(min, max), &new| {
        (min.min(new), max.max(new))
    })
}
