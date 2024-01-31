//! <https://www.codewars.com/kata/57a04da9e298a7ee43000111/train/rust>

pub fn reverse_list(lst: &[i32]) -> Vec<i32> {
    lst.iter().rev().copied().collect()
}
