//! <https://www.codewars.com/kata/59098c39d8d24d12b6000020/train/rust>

use unchecked_std::prelude::*;

pub fn dot(n: u32, m: u32) -> String {
    let mut res = String::with_capacity(6 * 3 * n as usize * m as usize);
    if n == 0 || m == 0 {
        return res;
    }
    unsafe {
        #[allow(clippy::range_plus_one)]
        for i in 0..m + 1 {
            for _ in 0..n {
                res.push_str_unchecked("+---");
            }
            res.push_unchecked('+');
            if i != m {
                res.push_unchecked('\n');
                for _ in 0..n {
                    res.push_str_unchecked("| o ");
                }
                res.push_str_unchecked("|\n");
            }
        }
    }
    res
}
