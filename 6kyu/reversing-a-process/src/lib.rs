//! <https://www.codewars.com/kata/5dad6e5264e25a001918a1fc/train/rust>

use core::mem::MaybeUninit;

pub fn decode(s: &str) -> String {
    let s = s.as_bytes();

    let mut num = 0;
    let mut i = 0;
    while s[i].is_ascii_digit() {
        num *= 10;
        num += (s[i] - b'0') as u64;

        i += 1;
        if i == s.len() {
            return String::new();
        }
    }

    if num % 2 == 0 || num % 13 == 0 {
        return "Impossible to decode".into();
    }

    let mut table = [MaybeUninit::uninit(); 26];
    for i in 0..26 {
        table[((i as u64 * num) % 26) as usize].write(b'a' + i);
    }
    let table = unsafe { table.assume_init_ref() };

    let bytes = s[i..].iter().map(|b| table[(b - b'a') as usize]).collect();
    unsafe { String::from_utf8_unchecked(bytes) }
}
