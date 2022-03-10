//! <https://www.codewars.com/kata/5f7c38eb54307c002a2b8cc8/train/rust>

pub fn remove_parentheses(s: &str) -> String {
    let mut res = Vec::with_capacity(s.len());

    let mut bytes = s.bytes();

    while let Some(b) = bytes.next() {
        if b == b'(' {
            let mut unclosed = 1usize;
            for b in bytes.by_ref() {
                match b {
                    b')' => unclosed -= 1,
                    b'(' => unclosed += 1,
                    _ => {}
                }

                if unclosed == 0 {
                    break;
                }
            }
        } else {
            res.push(b);
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
