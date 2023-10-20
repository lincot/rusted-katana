//! <https://www.codewars.com/kata/57c1ab3949324c321600013f/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn to_leet_speak(s: &str) -> String {
    let res = s
        .bytes()
        .map(|b| match b {
            b'A' => b'@',
            b'B' => b'8',
            b'C' => b'(',
            b'E' => b'3',
            b'G' => b'6',
            b'H' => b'#',
            b'I' => b'!',
            b'L' => b'1',
            b'O' => b'0',
            b'S' => b'$',
            b'T' => b'7',
            b'Z' => b'2',
            b => b,
        })
        .collect();
    unsafe { String::from_utf8_unchecked(res) }
}
