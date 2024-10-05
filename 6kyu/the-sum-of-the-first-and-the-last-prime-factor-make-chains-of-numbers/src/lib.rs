//! <https://www.codewars.com/kata/5629b94e34e04f8fb200008e/train/rust>

pub fn sflpf_data(val: u32, n_max: u32) -> Vec<u32> {
    let mut res = Vec::new();
    for n_orig in 4..n_max + 1 {
        let mut n = n_orig;

        let pow_of_2 = n.trailing_zeros();
        n >>= pow_of_2;

        let mut first_p_is_set = pow_of_2 != 0;
        let mut first_p = 2;
        let mut last_p = 2;

        let mut x = 3;
        let mut n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
        while x <= n_sqrt {
            let mut n_changed = false;

            while unsafe { n.checked_rem(x).unwrap_unchecked() } == 0 {
                n /= x;
                n_changed = true;
            }

            if n_changed {
                n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
                if !first_p_is_set {
                    first_p = x;
                    first_p_is_set = true;
                }
                last_p = x;
            }
            x += 2;
        }

        if n != 1 {
            last_p = n;
        }

        if n != n_orig && first_p + last_p == val {
            res.push(n_orig);
        }
    }
    res
}
