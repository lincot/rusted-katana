//! <https://www.codewars.com/kata/57f759bb664021a30300007d/train/rust>

pub fn switcheroo(s: &str) -> String {
    let res = s
        .bytes()
        .map(|b| {
            if b == b'a' {
                b'b'
            } else if b == b'b' {
                b'a'
            } else {
                b
            }
        })
        .collect();
    unsafe { String::from_utf8_unchecked(res) }
}
