//! <https://www.codewars.com/kata/56b5afb4ed1f6d5fb0000991/train/rust>

pub fn revrot(s: &str, c: usize) -> String {
    if c == 0 {
        return String::new();
    }

    let s = s.as_bytes();
    assert!(s.iter().all(|b| (b'0'..=b'9').contains(b)));

    let mut res = Vec::with_capacity(s.len());

    let mut end = c;
    while end <= s.len() {
        let chunk = unsafe { s.get_unchecked(end - c..end) };
        if chunk.iter().fold(0, |acc, &x| x.wrapping_add(acc)) % 2 == 0 {
            res.extend(chunk.iter().rev().copied());
        } else {
            res.extend(&chunk[1..]);
            res.push(chunk[0]);
        }

        end += c;
    }

    unsafe { String::from_utf8_unchecked(res) }
}
