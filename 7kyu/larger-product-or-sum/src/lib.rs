//! <https://www.codewars.com/kata/5c4cb8fc3cf185147a5bdd02/train/rust>

use std::cmp::Ordering;

pub fn sum_or_product(list: &[i64], n: usize) -> String {
    let mut list = list.to_vec();
    list.sort_unstable();

    let sum = list.iter().rev().take(n).sum::<i64>();
    let product = list.iter().take(n).product();

    match sum.cmp(&product) {
        Ordering::Greater => "sum",
        Ordering::Less => "product",
        Ordering::Equal => "same",
    }
    .into()
}
