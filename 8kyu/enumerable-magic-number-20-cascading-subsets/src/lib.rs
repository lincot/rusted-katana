//! <https://www.codewars.com/kata/545af3d185166a3dec001190/train/rust>

use core::mem::{transmute, MaybeUninit};

pub fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    let len = if n == 0 || arr.len() < n {
        0
    } else {
        arr.len() - n + 1
    };
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };
    let mut res_ptr = res.as_mut_ptr();
    for i in 0..len {
        unsafe {
            *res_ptr = MaybeUninit::new(arr.get_unchecked(i..i + n).to_vec());
            res_ptr = res_ptr.add(1);
        }
    }
    unsafe { transmute(res) }
}
