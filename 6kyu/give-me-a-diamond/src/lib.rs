//! <https://www.codewars.com/kata/5503013e34137eeeaa001648/train/rust>

use my_prelude::prelude::*;

pub fn print(n: i32) -> Option<String> {
    unsafe fn print_line(v: &mut Vec<u8>, width: usize, max_width: usize) {
        for _ in 0..(max_width - width) / 2 {
            v.push_unchecked(b' ');
        }
        for _ in 0..width {
            v.push_unchecked(b'*');
        }
        v.push_unchecked(b'\n');
    }

    if n < 0 || n % 2 == 0 {
        return None;
    }
    let n = n as usize;

    let cap = n.pow(2) + 1;
    let mut res = Vec::with_capacity(cap);

    let mut width = 1;
    while width <= n {
        unsafe { print_line(&mut res, width, n) };
        width += 2;
    }

    width = n;
    while width > 1 {
        width -= 2;
        unsafe { print_line(&mut res, width, n) };
    }

    Some(unsafe { String::from_utf8_unchecked(res) })
}
