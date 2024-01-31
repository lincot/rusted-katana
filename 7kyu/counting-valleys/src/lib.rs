//! <https://www.codewars.com/kata/5da9973d06119a000e604cb6/train/rust>

pub fn counting_valleys(s: &str) -> u32 {
    let mut level = 0i64;
    let mut res = 0;
    for &b in s.as_bytes() {
        match b {
            b'U' => {
                level += 1;
                if level == 0 {
                    res += 1;
                }
            }
            b'D' => level -= 1,
            _ => {}
        }
    }
    res
}
