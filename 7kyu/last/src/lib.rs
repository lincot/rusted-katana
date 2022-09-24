//! <https://www.codewars.com/kata/541629460b198da04e000bb9/train/rust>

#![no_std]

pub fn last<T: Clone>(slice: &[T]) -> T {
    slice.last().unwrap().clone()
}
