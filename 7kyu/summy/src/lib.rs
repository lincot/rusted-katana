//! <https://www.codewars.com/kata/599c20626bd8795ce900001d/train/rust>

pub fn summy(strng: &str) -> i32 {
    strng
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|s| {
            unsafe { core::str::from_utf8_unchecked(s) }
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}
