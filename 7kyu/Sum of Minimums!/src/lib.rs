//! <https://www.codewars.com/kata/5d5ee4c35162d9001af7d699/train/rust>

pub fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().map(|row| row.iter().min().unwrap()).sum()
}
