//! <https://www.codewars.com/kata/57d532d2164a67cded0001c7/train/rust>

use unchecked_std::prelude::*;

pub fn histogram(results: &[u32; 6]) -> String {
    let mut res = Vec::with_capacity("3| 100\n".len() * 6 + results.iter().sum::<u32>() as usize);
    for (i, &r) in (b'1'..=b'6').zip(results).rev() {
        unsafe {
            res.push_unchecked(i);
            res.push_unchecked(b'|');
            for _ in 0..r {
                res.push_unchecked(b'#');
            }
            if r != 0 {
                res.push_unchecked(b' ');
            }
            match r {
                0 => {}
                1..=9 => res.push_unchecked(b'0' + r as u8),
                100 => res.extend_from_slice_unchecked(b"100"),
                _ => {
                    res.push_unchecked(b'0' + (r / 10) as u8);
                    res.push_unchecked(b'0' + (r % 10) as u8);
                }
            }
            res.push_unchecked(b'\n');
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
