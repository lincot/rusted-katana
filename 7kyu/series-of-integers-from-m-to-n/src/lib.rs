//! <https://www.codewars.com/kata/5841f680c5c9b092950001ae/train/rust>

pub fn integer_series(m: u32, n: u32) -> Vec<u32> {
    let len = if n >= m { (n - m) as usize + 1 } else { 0 };
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };
    let mut res_ptr = res.as_mut_ptr();
    for i in 0..len {
        unsafe {
            *res_ptr = m + i as u32;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
