//! <https://www.codewars.com/kata/57f625992f4d53c24200070e/train/rust>

pub fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    let score = ticket
        .iter()
        .filter(|(s, n)| s.as_ref().as_bytes().contains(n))
        .count();
    if score >= win {
        "Winner!"
    } else {
        "Loser!"
    }
}
