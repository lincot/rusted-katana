//! <https://www.codewars.com/kata/65128732b5aff40032a3d8f0/train/rust>

pub fn neutralise(s1: &str, s2: &str) -> String {
    assert!(s1.is_ascii() && s1.len() == s2.len());
    let res = s1
        .bytes()
        .zip(s2.bytes())
        .map(|(a, b)| if a == b { a } else { b'0' })
        .collect();
    unsafe { String::from_utf8_unchecked(res) }
}
