//! <https://www.codewars.com/kata/5a00e05cc374cb34d100000d/train/rust>

pub fn reverse_seq(n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(n as _);
    unsafe { res.set_len(n as _) };
    let mut res_ptr = res.as_mut_ptr();
    let mut i = n;
    while i != 0 {
        unsafe {
            *res_ptr = i;
            res_ptr = res_ptr.add(1);
        }
        i -= 1;
    }
    res
}
