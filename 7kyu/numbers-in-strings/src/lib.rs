//! <https://www.codewars.com/kata/59dd2c38f703c4ae5e000014/train/rust>

pub fn solve(s: &str) -> u32 {
    let mut max = 0;
    let mut cur = 0;

    for b in s.bytes() {
        if b.is_ascii_digit() {
            cur *= 10;
            cur += (b - b'0') as u32;
        } else {
            max = max.max(cur);
            cur = 0;
        }
    }

    max.max(cur)
}
