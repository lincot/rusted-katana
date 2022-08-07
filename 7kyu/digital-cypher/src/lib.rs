//! <https://www.codewars.com/kata/592e830e043b99888600002d/train/rust>

use my_prelude::prelude::*;

pub fn encode(msg: String, n: i32) -> Vec<i32> {
    fn to_digits(mut n: i32) -> ([u8; 9], usize) {
        let (mut digits, mut len) = ([0; 9], 0);
        if n == 0 {
            len = 1;
        }
        while n != 0 {
            unsafe {
                *digits.get_unchecked_mut(len) = (n % 10) as u8;
            }
            n /= 10;
            len += 1;
        }
        (digits, len)
    }

    let (digits, len) = to_digits(n);

    let mut res = Vec::with_capacity(msg.len());
    let mut i = len - 1;

    for b in msg.as_bytes() {
        unsafe { res.push_unchecked((b + digits.get_unchecked(i) - b'a' + 1) as _) };
        if i == 0 {
            i = len - 1;
        } else {
            i -= 1;
        }
    }

    res
}
