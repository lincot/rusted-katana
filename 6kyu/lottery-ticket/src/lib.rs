//! <https://www.codewars.com/kata/57f625992f4d53c24200070e/train/rust>

#![no_std]

pub fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    if ticket
        .iter()
        .filter(|(s, n)| s.as_ref().as_bytes().iter().any(|b| b == n))
        .count()
        >= win
    {
        "Winner!"
    } else {
        "Loser!"
    }
}
