//! <https://www.codewars.com/kata/63ab271e96a48e000e577442/train/rust>

#![no_std]

pub fn can_escape(walls: &[(usize, usize)]) -> bool {
    (1..).zip(walls).all(|(i, &(a, b))| i < a && i < b)
}
