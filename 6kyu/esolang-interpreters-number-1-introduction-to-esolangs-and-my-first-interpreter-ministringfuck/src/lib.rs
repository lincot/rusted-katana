//! <https://www.codewars.com/kata/586dd26a69b6fd46dd0000c0/train/rust>

use unchecked_std::prelude::*;

pub fn my_first_interpreter(code: &str) -> String {
    let mut cell = 0u8;
    let mut res = Vec::with_capacity(code.len());
    for b in code.bytes() {
        if b == b'+' {
            cell = cell.wrapping_add(1);
        } else if b == b'.' {
            unsafe {
                if cell & (1 << 7) == 0 {
                    res.push_unchecked(cell);
                } else {
                    res.push_unchecked((cell as u32 >> 6 & 0x1F) as u8 | 0b1100_0000);
                    res.push_unchecked((cell as u32 & 0x3F) as u8 | 0b1000_0000);
                }
            }
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
