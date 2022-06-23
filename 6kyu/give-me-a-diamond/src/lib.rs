//! <https://www.codewars.com/kata/5503013e34137eeeaa001648/train/rust>

use std::iter::repeat;

pub fn print(n: i32) -> Option<String> {
    fn print_line(v: &mut Vec<u8>, width: usize, max_width: usize) {
        v.extend(repeat(b' ').take((max_width - width) / 2));
        v.extend(repeat(b'*').take(width));
        v.push(b'\n');
    }

    if n < 0 || n % 2 == 0 {
        return None;
    }
    let n = n as usize;

    let cap = n.pow(2) + 1;
    let mut res = Vec::with_capacity(cap);

    let mut width = 1;
    while width <= n {
        print_line(&mut res, width, n);
        width += 2;
    }

    width = n;
    while width > 1 {
        width -= 2;
        print_line(&mut res, width, n);
    }

    Some(unsafe { String::from_utf8_unchecked(res) })
}
