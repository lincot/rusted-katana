//! <https://www.codewars.com/kata/5656b6906de340bd1b0000ac/train/rust>

pub fn longest(a1: &str, a2: &str) -> String {
    let mut letters = [false; 26];

    for a in [a1, a2] {
        for b in a.bytes() {
            letters[(b - b'a') as usize] = true;
        }
    }

    let mut res = Vec::with_capacity(26);

    for (i, l) in (0..).zip(letters) {
        if l {
            res.push(b'a' + i);
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
