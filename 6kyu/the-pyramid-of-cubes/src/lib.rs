//! <https://www.codewars.com/kata/61707b71059070003793bc0f/train/rust>

#![no_std]

use num_integer::Roots;

pub fn find_height(cubes: usize) -> u16 {
    let res = (6 * cubes).cbrt() as u16;
    res - (6 * cubes < res as usize * (res as usize + 1) * (res as usize + 2)) as u16
}
