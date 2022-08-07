//! <https://www.codewars.com/kata/5bd776533a7e2720c40000e5/train/rust>

use my_prelude::prelude::*;

pub fn pendulum(xs: &[i32]) -> Vec<i32> {
    let mut xs = xs.to_vec();
    xs.sort_unstable();

    let mut res = Vec::with_capacity(xs.len());

    for i in (0..xs.len() - (xs.len() % 2 == 0) as usize)
        .rev()
        .step_by(2)
    {
        unsafe { res.push_unchecked(*xs.get_unchecked(i)) };
    }
    for i in (1..xs.len()).step_by(2) {
        unsafe { res.push_unchecked(*xs.get_unchecked(i)) };
    }

    res
}
