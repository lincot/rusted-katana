//! <https://www.codewars.com/kata/55902c5eaa8069a5b4000083/train/rust>

use digital::{Next2Digits, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn format_money(amount: f64) -> String {
    let mut res = String::with_capacity("$184467440737095516.15".len());
    let amount = amount.mul_add(100., 1e-7) as u64;
    unsafe {
        res.push_unchecked('$');
        write_with_cents(&mut res, amount);
    }
    res
}

unsafe fn write_with_cents(s: &mut String, mut amount: u64) {
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
