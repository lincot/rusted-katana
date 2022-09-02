//! <https://www.codewars.com/kata/596c6eb85b0f515834000049/train/rust>

pub fn replace_dots(s: &str) -> String {
    let mut s = s.to_string();
    for b in unsafe { s.as_bytes_mut() } {
        *b = if *b == b'.' { b'-' } else { *b };
    }
    s
}
