//! <https://www.codewars.com/kata/65127141a5de2b1dcb40927e/train/rust>

#![no_std]

pub fn spin_around(lst: &[&str]) -> u32 {
    let l = lst.iter().filter(|s| s.starts_with('l')).count();
    (l.abs_diff(lst.len() - l) / 4) as _
}
