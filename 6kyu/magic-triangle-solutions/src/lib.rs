//! <https://www.codewars.com/kata/62b2072d62c66500159693ff/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

const SETS: [[u8; 9]; 18] = [
    [1, 5, 9, 2, 4, 8, 3, 6, 7],
    [1, 5, 9, 4, 2, 6, 7, 3, 8],
    [1, 6, 8, 2, 5, 7, 3, 4, 9],
    [1, 6, 8, 4, 3, 5, 7, 2, 9],
    [1, 6, 8, 5, 2, 4, 9, 3, 7],
    [2, 4, 9, 5, 1, 6, 8, 3, 7],
    [2, 5, 9, 3, 1, 8, 7, 4, 6],
    [2, 6, 7, 5, 3, 4, 8, 1, 9],
    [2, 6, 8, 3, 4, 5, 7, 1, 9],
    [3, 2, 9, 7, 1, 5, 8, 4, 6],
    [3, 4, 8, 5, 2, 6, 7, 1, 9],
    [3, 4, 8, 6, 1, 5, 9, 2, 7],
    [3, 5, 6, 7, 2, 4, 8, 1, 9],
    [3, 5, 7, 6, 2, 4, 9, 1, 8],
    [4, 2, 9, 5, 1, 8, 6, 3, 7],
    [4, 3, 8, 5, 2, 7, 6, 1, 9],
    [7, 2, 6, 8, 1, 5, 9, 3, 4],
    [7, 3, 5, 8, 2, 4, 9, 1, 6],
];

const POSITIONS: [[u8; 6]; 6] = [
    [0, 3, 6, 1, 4, 7],
    [0, 6, 3, 7, 4, 1],
    [3, 0, 6, 1, 7, 4],
    [3, 6, 0, 4, 7, 1],
    [6, 0, 3, 7, 1, 4],
    [6, 3, 0, 4, 1, 7],
];

pub fn magic_triangle_solutions(puzzle: &[u8; 9]) -> Vec<[u8; 9]> {
    let mut res = Vec::with_capacity((SETS.len() * POSITIONS.len()) << 3);
    unsafe {
        for set in SETS {
            for positions in POSITIONS {
                if [0, *set.get_unchecked(positions[0] as usize)].contains(&puzzle[0])
                    && [0, *set.get_unchecked(positions[1] as usize)].contains(&puzzle[3])
                    && [0, *set.get_unchecked(positions[2] as usize)].contains(&puzzle[6])
                {
                    for (pos_1, pos_2) in [
                        (positions[3], positions[3] + 1),
                        (positions[3] + 1, positions[3]),
                    ] {
                        if [0, *set.get_unchecked(pos_1 as usize)].contains(&puzzle[1])
                            && [0, *set.get_unchecked(pos_2 as usize)].contains(&puzzle[2])
                        {
                            for (pos_4, pos_5) in [
                                (positions[4], positions[4] + 1),
                                (positions[4] + 1, positions[4]),
                            ] {
                                if [0, *set.get_unchecked(pos_4 as usize)].contains(&puzzle[4])
                                    && [0, *set.get_unchecked(pos_5 as usize)].contains(&puzzle[5])
                                {
                                    for (pos_7, pos_8) in [
                                        (positions[5], positions[5] + 1),
                                        (positions[5] + 1, positions[5]),
                                    ] {
                                        if [0, *set.get_unchecked(pos_7 as usize)]
                                            .contains(&puzzle[7])
                                            && [0, *set.get_unchecked(pos_8 as usize)]
                                                .contains(&puzzle[8])
                                        {
                                            res.push_unchecked([
                                                *set.get_unchecked(positions[0] as usize),
                                                *set.get_unchecked(pos_1 as usize),
                                                *set.get_unchecked(pos_2 as usize),
                                                *set.get_unchecked(positions[1] as usize),
                                                *set.get_unchecked(pos_4 as usize),
                                                *set.get_unchecked(pos_5 as usize),
                                                *set.get_unchecked(positions[2] as usize),
                                                *set.get_unchecked(pos_7 as usize),
                                                *set.get_unchecked(pos_8 as usize),
                                            ]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    res
}
