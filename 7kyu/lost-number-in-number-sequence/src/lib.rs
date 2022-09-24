//! <https://www.codewars.com/kata/595aa94353e43a8746000120/train/rust>

#![no_std]

pub fn find_deleted_number(list: &[u16], mixed_list: &[u16]) -> Option<u16> {
    let len = list.len() as u16;
    let x = len * (len + 1) / 2 - mixed_list.iter().sum::<u16>();

    if x == 0 {
        None
    } else {
        Some(x)
    }
}
