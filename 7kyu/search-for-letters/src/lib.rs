//! <https://www.codewars.com/kata/52dbae61ca039685460001ae/train/rust>

pub fn change(string: &str) -> String {
    let mut res = b"00000000000000000000000000".to_vec();

    for b in string.bytes() {
        if (b'a'..=b'z').contains(&b) {
            res[(b - b'a') as usize] = b'1';
        } else if (b'A'..=b'Z').contains(&b) {
            res[(b - b'A') as usize] = b'1';
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
