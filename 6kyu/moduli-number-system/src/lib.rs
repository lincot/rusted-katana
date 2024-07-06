//! <https://www.codewars.com/kata/54db15b003e88a6a480000b9/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use num_integer::gcd;
use unchecked_std::prelude::*;

pub fn from_nb_2str(n: i64, sys: Vec<i64>) -> String {
    let Some(mut product) = sys.first().copied() else {
        return "Not applicable".into();
    };
    for &m in &sys[1..] {
        if gcd(product, m) != 1 {
            return "Not applicable".into();
        }
        product *= m;
    }
    if product <= n {
        return "Not applicable".into();
    }

    let mut res = String::with_capacity((2 + i64::MAX_LEN_BASE10) * sys.len());
    for m in sys {
        unsafe {
            res.push_unchecked('-');
            res.write_num_unchecked(n % m, 10, false, false);
            res.push_unchecked('-');
        }
    }
    res
}
