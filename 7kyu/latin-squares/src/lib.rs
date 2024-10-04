//! <https://www.codewars.com/kata/645fb55ecf8c290031b779ef/train/rust>

use unchecked_std::prelude::*;

pub fn make_latin_square(n: i32) -> Vec<Vec<i32>> {
    #[allow(clippy::range_plus_one)]
    (1..n + 1)
        .map(|i| {
            let mut res = Vec::with_capacity(n as usize);
            unsafe {
                #[allow(clippy::range_plus_one)]
                for x in i..n + 1 {
                    res.push_unchecked(x);
                }
                for x in 1..i {
                    res.push_unchecked(x);
                }
            }
            res
        })
        .collect()
}
