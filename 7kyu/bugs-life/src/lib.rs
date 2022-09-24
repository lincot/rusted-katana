//! <https://www.codewars.com/kata/5b71af678adeae41df00008c/train/rust>

#![no_std]

use libm::hypot;

pub fn shortest_distance(a: f64, b: f64, c: f64) -> f64 {
    let largest = a.max(b).max(c);
    let sum_others = a + b + c - largest;
    hypot(sum_others, largest)
}
