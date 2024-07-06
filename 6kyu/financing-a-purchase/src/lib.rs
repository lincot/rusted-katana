//! <https://www.codewars.com/kata/59c68ea2aeb2843e18000109/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn amort(rate: f64, balance: i64, term: i64, num_payments: i32) -> String {
    let r = rate / 100. / 12.;
    let c = (r * balance as f64) / (1. - (1. + r).powi(-term as i32));
    let dn = (1. + r).powi(num_payments - 1);
    let balance_before_payment = dn.mul_add(balance as f64, (c / r) * (1. - dn));
    let int = balance_before_payment * r;
    let princ = c - int;
    let balance = balance_before_payment - princ;

    let mut res = String::with_capacity(
        "num_payment  c  princ  int  balance ".len()
            + i32::MAX_LEN_BASE10
            + 4 * u64::MAX_LEN_BASE10,
    );
    unsafe {
        res.push_str_unchecked("num_payment ");
        res.write_num_unchecked(num_payments, 10, false, false);
        res.push_str_unchecked(" c ");
        res.write_num_unchecked((c + 0.5) as u64, 10, false, false);
        res.push_str_unchecked(" princ ");
        res.write_num_unchecked((princ + 0.5) as u64, 10, false, false);
        res.push_str_unchecked(" int ");
        res.write_num_unchecked((int + 0.5) as u64, 10, false, false);
        res.push_str_unchecked(" balance ");
        res.write_num_unchecked((balance + 0.5) as u64, 10, false, false);
    }
    res
}
