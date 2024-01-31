//! <https://www.codewars.com/kata/5596f6e9529e9ab6fb000014/train/rust>

pub fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    if first.len() == second.len() {
        for i in 0..first.len() {
            if first.as_bytes().starts_with(&second.as_bytes()[i..])
                && first.as_bytes().ends_with(&second.as_bytes()[..i])
            {
                return Some(i);
            }
        }
    }
    None
}
