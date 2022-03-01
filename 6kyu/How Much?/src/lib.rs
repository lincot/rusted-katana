//! <https://www.codewars.com/kata/55b4d87a3766d9873a0000d4/train/rust>

use std::mem::swap;

pub fn how_much(mut m: i32, mut n: i32) -> Vec<(String, String, String)> {
    if m > n {
        swap(&mut m, &mut n);
    }

    ((m + 25) / 63..(n + 26) / 63)
        .map(|k| {
            (
                format!("{}: {}", 'M', k * 63 + 37),
                format!("{}: {}", 'B', k * 9 + 5),
                format!("{}: {}", 'C', k * 7 + 4),
            )
        })
        .collect()
}
