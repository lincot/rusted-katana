//! <https://www.codewars.com/kata/5592e3bd57b64d00f3000047/train/rust>

pub fn find_nb(m: u64) -> i32 {
    let t = (m as f64).sqrt().mul_add(8., 1.).sqrt();
    let n = unsafe { (t.to_int_unchecked::<u64>() - 1) / 2 };
    if (n * n + n).pow(2) / 4 == m {
        n as _
    } else {
        -1
    }
}
