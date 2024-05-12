//! <https://www.codewars.com/kata/596549c7743cf369b900021b/train/rust>

use num_prime::nt_funcs::is_prime64;

pub fn twin_prime(n: i32) -> u32 {
    if n <= 3 {
        return 0;
    }
    let n = n as u32;

    let mut p = 5;
    let mut res = 1;

    while p < n {
        if is_prime64(p as _) && is_prime64((p + 2) as _) {
            res += 1;
        }
        p += 6;
    }

    res
}
