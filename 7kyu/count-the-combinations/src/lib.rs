//! <https://www.codewars.com/kata/58e67378fd2d897b8a000110/train/rust>

pub fn num_combo(array: &[u32], number: u32) -> usize {
    array
        .iter()
        .sum::<u32>()
        .checked_sub(number)
        .map_or(0, |d| array.iter().filter(|&&x| x == d).count())
}
