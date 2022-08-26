//! <https://www.codewars.com/kata/5a4ea304b3bfa89a9900008e/train/rust>

use my_prelude::prelude::*;

pub fn max_number(n: u32) -> u32 {
    fn to_digits(n: u32) -> heapless::Vec<u8, 10> {
        let mut digits = heapless::Vec::new();
        // TODO: convert to digits directly
        unsafe { digits.write_num_unchecked(n) };
        digits
    }

    let mut digits = to_digits(n);
    digits.sort_unstable();
    digits
        .iter()
        .rev()
        .fold(0, |acc, &d| 10 * acc + (d - b'0') as u32)
}
