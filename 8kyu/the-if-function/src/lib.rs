//! <https://www.codewars.com/kata/54147087d5c2ebe4f1000805/train/rust>

#![no_std]

pub fn _if<T, F1, F2>(cond: bool, mut then: F1, mut els: F2) -> T
where
    F1: FnMut() -> T,
    F2: FnMut() -> T,
{
    if cond {
        then()
    } else {
        els()
    }
}
