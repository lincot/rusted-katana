//! <https://www.codewars.com/kata/568dc69683322417eb00002c/train/rust>

pub fn triple_x(s: &str) -> bool {
    let mut bytes = s.bytes();

    for b in bytes.by_ref() {
        if b == b'x' {
            return bytes.next() == Some(b'x') && bytes.next() == Some(b'x');
        }
    }

    false
}
