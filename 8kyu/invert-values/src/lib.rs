//! <https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/rust>

pub fn invert(values: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(values.len());
    unsafe { res.set_len(values.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for &x in values {
        unsafe {
            *res_ptr = -x;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
