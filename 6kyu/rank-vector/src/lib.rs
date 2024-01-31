//! <https://www.codewars.com/kata/545f05676b42a0a195000d95/train/rust>

use core::cmp::Reverse;

pub fn ranks(a: &[i32]) -> Vec<usize> {
    if a.is_empty() {
        return Vec::new();
    }

    let mut a_enumerated: Vec<_> = a.iter().copied().enumerate().collect();
    if a_enumerated.len() < 64 {
        a_enumerated.sort_unstable_by_key(|&(_, x)| Reverse(x));
    } else {
        radsort::sort_by_key(&mut a_enumerated, |&(_, x)| -x);
    }

    let mut res = Vec::with_capacity(a.len());
    unsafe { res.set_len(a.len()) };
    let mut rank = 1;

    let &(i, mut prev) = a_enumerated.first().unwrap();
    unsafe { *res.get_unchecked_mut(i) = rank };

    let mut same_count = 0;
    for &(i, x) in &a_enumerated[1..] {
        same_count += 1;
        if x != prev {
            rank += same_count;
            same_count = 0;
        }
        unsafe { *res.get_unchecked_mut(i) = rank };
        prev = x;
    }

    res
}
