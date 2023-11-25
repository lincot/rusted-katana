//! <https://www.codewars.com/kata/61488fde47472d000827a51d/train/rust>

#![no_std]

pub fn is_valid(positions: &str) -> bool {
    let positions = positions.as_bytes();
    if positions.len() != 8 {
        return false;
    }

    let Some(bishop_left) = positions.iter().position(|&b| b == b'B') else {
        return false;
    };
    let Some(bishop_right) = positions.iter().rposition(|&b| b == b'B') else {
        return false;
    };
    let Some(rook_left) = positions.iter().position(|&b| b == b'R') else {
        return false;
    };
    let Some(rook_right) = positions.iter().rposition(|&b| b == b'R') else {
        return false;
    };
    let Some(king) = positions.iter().position(|&b| b == b'K') else {
        return false;
    };

    bishop_left % 2 != bishop_right % 2 && (rook_left + 1..rook_right).contains(&king)
}
