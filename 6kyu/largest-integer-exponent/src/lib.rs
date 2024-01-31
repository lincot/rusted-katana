//! <https://www.codewars.com/kata/59b139d69c56e8939700009d/train/rust>

pub const fn get_exponent(n: i32, p: u32) -> Option<u32> {
    if p <= 1 {
        return None;
    }
    let n = n.unsigned_abs();
    let mut res = 0;
    let mut p_exp = p;
    while n % p_exp == 0 {
        p_exp *= p;
        res += 1;
    }
    Some(res)
}
