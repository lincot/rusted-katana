//! <https://www.codewars.com/kata/69e3d400196a47ceacde554f/train/rust>

use digital::prelude::*;
use num_integer::Integer;
use unchecked_std::prelude::*;

pub fn merge_ratios(ratio1: &str, ratio2: &str) -> String {
    let (a, b) = parse_ratio(ratio1);
    let (c, d) = parse_ratio(ratio2);
    let r = [a * c, b * c, b * d];
    let g = r[0].gcd(&r[1].gcd(&r[2]));
    let [a, b, c] = r.map(|x| x / g);

    let mut res = String::with_capacity(3 * u64::MAX_LEN_BASE10 + 2);
    unsafe {
        res.write_int_unchecked(a);
        res.push_unchecked(':');
        res.write_int_unchecked(b);
        res.push_unchecked(':');
        res.write_int_unchecked(c);
    }
    res
}

const fn parse_ratio(ratio: &str) -> (u32, u32) {
    let ratio = ratio.as_bytes();

    let (mut a, mut b) = (0, 0);

    let mut i = 0;
    while i < ratio.len() {
        if ratio[i] == b':' {
            i += 1;
            break;
        }

        a *= 10;
        a += (ratio[i] - b'0') as u32;
        i += 1;
    }

    while i < ratio.len() {
        b *= 10;
        b += (ratio[i] - b'0') as u32;
        i += 1;
    }

    (a, b)
}
