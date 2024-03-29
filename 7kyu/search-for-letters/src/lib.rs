//! <https://www.codewars.com/kata/52dbae61ca039685460001ae/train/rust>

pub fn change(string: &str) -> String {
    let mut res = "00000000000000000000000000".to_string();
    for b in string.bytes() {
        unsafe {
            if b.is_ascii_lowercase() {
                res.as_mut_vec()[(b - b'a') as usize] = b'1';
            } else if b.is_ascii_uppercase() {
                res.as_mut_vec()[(b - b'A') as usize] = b'1';
            }
        }
    }
    res
}
