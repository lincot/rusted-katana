//! <https://www.codewars.com/kata/58223370aef9fc03fd000071/train/rust>

use my_prelude::prelude::*;

pub fn dashatize(mut n: i64) -> String {
    fn to_digits(mut n: i64) -> ([u8; 19], usize) {
        let (mut digits, mut len) = ([0; 19], 0);
        while n != 0 {
            unsafe { *digits.get_unchecked_mut(len) = (n % 10) as u8 + b'0' };
            n /= 10;
            len += 1;
        }
        if len > digits.len() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        (digits, len)
    }

    if n == 0 {
        return "0".into();
    }
    if n == i64::MIN {
        return "9-22-3-3-7-20-3-68-5-4-7-7-5-808".into();
    }
    if n < 0 {
        n = -n;
    }

    let (digits, digits_len) = to_digits(n);
    if digits_len == 0 {
        unsafe { core::hint::unreachable_unchecked() };
    }

    let mut res = Vec::with_capacity(2 * digits_len);

    let mut digits = digits[..digits_len].iter().rev();

    let first = *digits.next().unwrap();
    unsafe { res.push_unchecked(first) };
    let mut was_odd = first % 2 == 1;

    for &d in digits {
        let is_odd = d % 2 == 1;

        if was_odd || is_odd {
            unsafe { res.push_unchecked(b'-') };
        }
        unsafe { res.push_unchecked(d) };

        was_odd = is_odd;
    }

    unsafe { String::from_utf8_unchecked(res) }
}
