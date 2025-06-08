//! <https://www.codewars.com/kata/5ffc226ce1666a002bf023d2/train/rust>

pub fn solution(mtrx: &[Vec<char>]) -> bool {
    mtrx.iter().any(|row| {
        row.len() <= 7
            && row
                .iter()
                .position(|&c| c == '>')
                .is_some_and(|i| row[i + 1..].contains(&'x'))
    })
}
