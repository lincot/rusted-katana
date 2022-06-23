//! <https://www.codewars.com/kata/56541980fa08ab47a0000040/train/rust>

pub fn printer_error(s: &str) -> String {
    let malformed = s.bytes().filter(|&b| b > b'm').count();
    let all = s.len();

    format!("{}/{}", malformed, all)
}
