//! <https://www.codewars.com/kata/5a58d889880385c2f40000aa/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

const AUTOMORPHIC_NUMBERS: [u64; 28] = [
    0,
    1,
    5,
    6,
    25,
    76,
    376,
    625,
    9_376,
    90_625,
    109_376,
    890_625,
    2_890_625,
    7_109_376,
    12_890_625,
    87_109_376,
    212_890_625,
    787_109_376,
    1_787_109_376,
    8_212_890_625,
    18_212_890_625,
    81_787_109_376,
    918_212_890_625,
    9_918_212_890_625,
    40_081_787_109_376,
    59_918_212_890_625,
    259_918_212_890_625,
    740_081_787_109_376,
];

pub fn automorphic(n: u64) -> String {
    if AUTOMORPHIC_NUMBERS.contains(&n) {
        "Automorphic"
    } else {
        "Not!!"
    }
    .into()
}
