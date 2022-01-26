//! <https://www.codewars.com/kata/568dcc3c7f12767a62000038/train/rust>

pub const fn set_alarm(employed: bool, vacation: bool) -> bool {
    employed && !vacation
}
