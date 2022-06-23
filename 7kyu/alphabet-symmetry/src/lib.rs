//! <https://www.codewars.com/kata/59d9ff9f7905dfeed50000b0/train/rust>

pub fn solve(strings: &[String]) -> Vec<usize> {
    strings
        .iter()
        .map(|string| {
            (b'a'..=b'z')
                .zip(string.bytes())
                .filter(|&(i, b)| i == b || i - b'a' + b'A' == b)
                .count()
        })
        .collect()
}
