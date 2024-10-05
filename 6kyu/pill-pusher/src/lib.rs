//! <https://www.codewars.com/kata/628e6f112324192c65cd8c97/train/rust>

pub fn prescribe(d: u16, a: u16, b: u16) -> u16 {
    let mut res = d % a;
    let mut r = res;
    let ba = b % a;
    for _ in 0..d / b + 1 {
        if r < ba {
            res = res.min(r);
            r += a;
        }
        r -= ba;
    }
    d - res.min(r + ba)
}
