//! <https://www.codewars.com/kata/542c0f198e077084c0000c2e/train/rust>

pub fn divisors(mut n: u32) -> u32 {
    let pow_of_2 = n.trailing_zeros();
    n >>= pow_of_2;
    let mut res = pow_of_2 + 1;

    let mut x = 3;
    let mut n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
    while x <= n_sqrt {
        let mut n_changed = false;
        let mut p = 1;

        while unsafe { n.checked_rem(x).unwrap_unchecked() } == 0 {
            n /= x;
            n_changed = true;
            p += 1;
        }

        if p != 1 {
            res *= p;
        }

        x += 2;
        if n_changed {
            n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
        }
    }

    if n != 1 {
        res *= 2;
    }
    res
}
