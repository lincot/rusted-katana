//! <https://www.codewars.com/kata/5977618080ef220766000022/train/rust>

use digital::{Next2Digits, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn usdcny(usd: u16) -> String {
    let mut res = String::with_capacity("442361.25 Chinese Yuan".len());
    unsafe {
        write_with_cents(&mut res, usd as u32 * 675);
        res.push_str_unchecked(" Chinese Yuan");
    }
    res
}

unsafe fn write_with_cents(s: &mut String, mut amount: u32) {
    let last_2 = amount.next_2_digits(false).unwrap_or_else(|| {
        let d = amount;
        amount = 0;
        [b'0', d as u8 + b'0']
    });
    s.write_num_unchecked(amount, 10, false, false);
    s.as_mut_vec().push_unchecked(b'.');
    s.as_mut_vec().push_unchecked(last_2[0]);
    s.as_mut_vec().push_unchecked(last_2[1]);
}
