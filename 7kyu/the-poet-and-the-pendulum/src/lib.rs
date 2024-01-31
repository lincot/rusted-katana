//! <https://www.codewars.com/kata/5bd776533a7e2720c40000e5/train/rust>

use unchecked::PushUnchecked;
use vqsort::VqSort;

pub fn pendulum(xs: &[i32]) -> Vec<i32> {
    let mut xs = xs.to_vec();
    VqSort::sort_ascending(&mut xs);
    let mut res = Vec::with_capacity(xs.len());
    let mut i = (xs.len() - (xs.len() % 2 == 0) as usize).wrapping_sub(1);
    for _ in 0..(i + 2) / 2 {
        unsafe { res.push_unchecked(*xs.get_unchecked(i)) };
        i = i.wrapping_sub(2);
    }
    let mut i = 1;
    for _ in 0..xs.len() / 2 {
        unsafe { res.push_unchecked(*xs.get_unchecked(i)) };
        i += 2;
    }
    res
}
