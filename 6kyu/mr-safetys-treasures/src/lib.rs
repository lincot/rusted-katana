//! <https://www.codewars.com/kata/592c1dfb912f22055b000099/train/rust>

pub fn unlock(message: &str) -> String {
    let bytes = message
        .bytes()
        .map(|b| {
            let b = (b | 0x20) - b'a';
            let b = (b - (b > 15) as u8).min(22);
            b'0' + b / 3 + 2
        })
        .collect();
    unsafe { String::from_utf8_unchecked(bytes) }
}
