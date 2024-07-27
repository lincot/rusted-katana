//! <https://www.codewars.com/kata/5626ec066d35051d4500009e/train/rust>

pub fn mult_primefactor_sum(a: u32, b: u32) -> Vec<u32> {
    let mut res = Vec::new();
    for n_orig in a..=b {
        let mut n = n_orig;

        let pow_of_2 = n.trailing_zeros();
        n >>= pow_of_2;
        let mut s = 2 * pow_of_2;

        let mut x = 3;
        let mut n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
        while x <= n_sqrt {
            let mut n_changed = false;

            while unsafe { n.checked_rem(x).unwrap_unchecked() } == 0 {
                n /= x;
                n_changed = true;
                s += x;
            }

            x += 2;
            if n_changed {
                n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
            }
        }

        if s != 0 {
            if n != 1 {
                s += n;
            }
            if n_orig % s == 0 {
                res.push(n_orig);
            }
        }
    }
    res
}
