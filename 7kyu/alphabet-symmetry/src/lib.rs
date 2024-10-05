//! <https://www.codewars.com/kata/59d9ff9f7905dfeed50000b0/train/rust>

pub fn solve(strings: &[String]) -> Vec<usize> {
    strings
        .iter()
        .map(|string| {
            (b'a'..b'z' + 1)
                .zip(b'A'..b'Z' + 1)
                .zip(string.as_bytes())
                .filter(|&((l, u), &b)| b == l || b == u)
                .count()
        })
        .collect()
}
