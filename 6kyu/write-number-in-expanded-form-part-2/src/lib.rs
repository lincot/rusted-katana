//! <https://www.codewars.com/kata/58cda88814e65627c5000045/train/rust>

use unchecked_std::prelude::*;

pub fn expanded_form(num: f64) -> String {
    let mut digits = ryu::Buffer::new();
    let digits = digits.format_finite(num).as_bytes();
    let mut res = Vec::with_capacity(((digits.len() - 1) * (digits.len() + 10) / 2 - 3) as _);
    let dot_pos = digits
        .iter()
        .position(|&b| b == b'.')
        .unwrap_or(digits.len());
    let mut first = true;
    unsafe {
        for (i, &d) in (0..dot_pos).rev().zip(digits).filter(|(_, &b)| b != b'0') {
            if !first {
                res.extend_from_slice_unchecked(b" + ");
            }
            first = false;
            res.push_unchecked(d);
            for _ in 0..i {
                res.push_unchecked(b'0');
            }
        }
        if dot_pos < digits.len() - 1 {
            for (i, &d) in (1..)
                .zip(&digits[dot_pos + 1..])
                .filter(|(_, &b)| b != b'0')
            {
                if !first {
                    res.extend_from_slice_unchecked(b" + ");
                }
                first = false;
                res.push_unchecked(d);
                res.extend_from_slice_unchecked(b"/1");
                for _ in 0..i {
                    res.push_unchecked(b'0');
                }
            }
        }
        String::from_utf8_unchecked(res)
    }
}
