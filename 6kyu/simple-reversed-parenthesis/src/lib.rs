//! <https://www.codewars.com/kata/5a3f2925b6cfd78fb0000040/train/rust>

pub fn solve(s: &str) -> Option<usize> {
    if s.len() % 2 == 1 {
        return None;
    }

    let mut unclosed = 0;
    let mut unopened = 0;

    for b in s.bytes() {
        if b == b'(' {
            unclosed += 1;
        } else if unclosed == 0 {
            unopened += 1;
        } else {
            unclosed -= 1;
        }
    }

    Some(unclosed / 2 + unclosed % 2 + unopened / 2 + unopened % 2)
}
