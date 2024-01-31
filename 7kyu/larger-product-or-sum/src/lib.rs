//! <https://www.codewars.com/kata/5c4cb8fc3cf185147a5bdd02/train/rust>

use core::cmp::Ordering;
use vqsort::VqSort;

pub fn sum_or_product(list: &[i64], n: usize) -> String {
    let mut list = list.to_vec();

    list.select_nth_unstable(n.saturating_sub(1));
    VqSort::sort_ascending(unsafe { list.get_unchecked_mut(..n) });
    let product = list.iter().take(n).product();

    let len = list.len();
    list.select_nth_unstable(len - n);
    VqSort::sort_ascending(unsafe { list.get_unchecked_mut(len - n..) });
    let sum = list.iter().rev().take(n).sum::<i64>();

    match sum.cmp(&product) {
        Ordering::Greater => "sum".into(),
        Ordering::Less => "product".into(),
        Ordering::Equal => "same".into(),
    }
}
