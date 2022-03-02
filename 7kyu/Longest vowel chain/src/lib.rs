//! <https://www.codewars.com/kata/59c5f4e9d751df43cf000035/train/rust>

pub fn longest_vowel_chain(s: &str) -> usize {
    let mut max = 0;
    let mut cur = 0;

    for b in s.bytes() {
        if b"eaiou".contains(&b) {
            cur += 1;
        } else {
            max = max.max(cur);
            cur = 0;
        }
    }

    max.max(cur)
}
