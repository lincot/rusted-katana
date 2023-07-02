//! <https://www.codewars.com/kata/6177b4119b69a40034305f14/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn solution(mtrx: &[Vec<char>]) -> bool {
    let mut x_pos = None;
    for (i, row) in mtrx.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                'x' => x_pos = Some((i, j)),
                '>' => {
                    return x_pos.is_none() && row.get(j + 1..).is_some_and(|s| s.contains(&'x'));
                }
                '<' => {
                    return x_pos.is_some_and(|(x, _)| x == i);
                }
                '^' => {
                    return x_pos.is_some_and(|(_, y)| y == j);
                }
                'v' => {
                    return mtrx
                        .get(i + 1..)
                        .is_some_and(|rows| rows.iter().any(|x| x.get(j) == Some(&'x')));
                }
                _ => {}
            }
        }
    }
    false
}
