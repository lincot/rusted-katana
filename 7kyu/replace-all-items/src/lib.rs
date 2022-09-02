//! <https://www.codewars.com/kata/57ae18c6e298a7a6d5000c7a/train/rust>

use core::mem::{transmute, MaybeUninit};

pub fn replace_all<T: PartialEq + Copy>(xs: &[T], find: T, replace: T) -> Vec<T> {
    let mut res = Vec::with_capacity(xs.len());
    unsafe { res.set_len(xs.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for &x in xs {
        unsafe {
            *res_ptr = MaybeUninit::new(if x == find { replace } else { x });
            res_ptr = res_ptr.add(1);
        }
    }
    unsafe { transmute(res) }
}
