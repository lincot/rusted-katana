//! <https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa/train/rust>

use core::mem::{transmute, MaybeUninit};

pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut res = Vec::with_capacity(data.len());
    unsafe { res.set_len(data.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for (age, handicap) in data {
        unsafe {
            *res_ptr = MaybeUninit::new(if age >= 55 && handicap > 7 {
                "Senior".to_string()
            } else {
                "Open".to_string()
            });
            res_ptr = res_ptr.add(1);
        }
    }
    unsafe { transmute(res) }
}
