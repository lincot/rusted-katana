//! <https://www.codewars.com/kata/545cedaa9943f7fe7b000048/train/rust>

pub fn is_pangram(s: &str) -> bool {
    let mut seen = 0;
    for b in s.bytes() {
        let raw = (b | 0x20).wrapping_sub(b'a');
        if raw < 26 {
            seen |= 1 << raw;
        }
    }
    seen == (1 << 26) - 1
}
