//! <https://www.codewars.com/kata/542f3d5fd002f86efc00081a/train/rust>

pub fn prime_factors(mut n: u32) -> Vec<u32> {
    let pow_of_2 = n.trailing_zeros();
    n >>= pow_of_2;
    let mut res = vec![2; pow_of_2 as usize];

    let mut x = 3;
    let mut n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
    while x <= n_sqrt {
        let mut n_changed = false;

        while unsafe { n.checked_rem(x).unwrap_unchecked() } == 0 {
            n /= x;
            n_changed = true;
            res.push(x);
        }

        x += 2;
        if n_changed {
            n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
        }
    }

    if n != 1 {
        res.push(n);
    }

    res
}
