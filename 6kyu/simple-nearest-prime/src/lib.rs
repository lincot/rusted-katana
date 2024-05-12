//! <https://www.codewars.com/kata/5a9078e24a6b340b340000b8/train/rust>

use num_prime::nt_funcs::is_prime64;

pub fn solve(n: u64) -> u64 {
    match n {
        0..=2 => return 2,
        3..=4 => return 3,
        _ => {}
    }

    let r = n % 6;
    let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2) };
    let left_is_closer = [0, 2, 3].contains(&r);
    let mut right = n + next_r - r;
    let mut left = right - (step ^ 6);

    loop {
        if left_is_closer && is_prime64(left) {
            return left;
        }

        if is_prime64(right) {
            if r == 2 && step == 2 && is_prime64(left - step) {
                return left - step;
            }
            return right;
        }

        if !left_is_closer && is_prime64(left) {
            if r == 5 && step == 2 && is_prime64(right + step) {
                return right + step;
            }
            return left;
        }

        left -= step;
        right += step;
        step ^= 6;
    }
}
