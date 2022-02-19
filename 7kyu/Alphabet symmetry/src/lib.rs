//! <https://www.codewars.com/kata/59d9ff9f7905dfeed50000b0/train/rust>

pub fn solve(strings: &[String]) -> Vec<usize> {
    strings
        .iter()
        .map(|string| {
            (b'a'..b'z')
                .zip(string.bytes())
                .filter(|&(i, b)| {
                    (b'a'..=b'z').contains(&b) && i == b
                        || (b'A'..=b'Z').contains(&b) && i == b - b'A' + b'a'
                })
                .count()
        })
        .collect()
}
