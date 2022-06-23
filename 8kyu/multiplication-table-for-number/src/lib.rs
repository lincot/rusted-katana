//! <https://www.codewars.com/kata/5a2fd38b55519ed98f0000ce/train/rust>

use std::fmt::Write;

pub fn multi_table(n: u64) -> String {
    let repeating = format!(" * {} = ", n);

    // worst case capacity, may be 8 less
    let cap = 20 * repeating.len() - 31;
    let mut res = String::with_capacity(cap);

    write!(
        res,
        "1{repeating}{}
2{repeating}{}
3{repeating}{}
4{repeating}{}
5{repeating}{}
6{repeating}{}
7{repeating}{}
8{repeating}{}
9{repeating}{}
10{repeating}{}",
        n,
        2 * n,
        3 * n,
        4 * n,
        5 * n,
        6 * n,
        7 * n,
        8 * n,
        9 * n,
        10 * n,
    )
    .unwrap();

    res
}
