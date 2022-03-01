//! <https://www.codewars.com/kata/5613d06cee1e7da6d5000055/train/rust>

/// checks if (!(`x` is divisible by 2 or 3) && `x` is prime) || `x` <= 3
fn is_prime_with_condition(x: u64) -> bool {
    let mut divisor = 5;
    let mut step = 2;
    while divisor <= (x as f64).sqrt() as u64 {
        if divisor == 0 {
            unsafe { core::hint::unreachable_unchecked() };
        }
        if x % divisor == 0 {
            return false;
        }

        divisor += step;
        step ^= 6;
    }

    true
}

fn step_divisible_by_6(g: u64, mut m: u64, n: u64) -> Option<(u64, u64)> {
    let r = m % 6;
    let mut step = if r <= 1 {
        m += 1 - r;
        4
    } else {
        m += 5 - r;
        2
    };

    while m <= n {
        if is_prime_with_condition(m) && is_prime_with_condition(m + g) {
            return Some((m, m + g));
        }

        m += step;
        step ^= 6;
    }

    None
}

pub fn step(g: i32, mut m: u64, n: u64) -> Option<(u64, u64)> {
    let g = g.abs() as u64;
    let n = n - g;
    let g6 = g % 6;

    if m <= 2 && n <= 2 {
        if g <= 1 || [5, 3].contains(&g6) && is_prime_with_condition(2 + g) {
            return Some((2, 2 + g));
        }
        m = 3;
    }

    if m <= 3 && n <= 3 {
        if g == 0 || [4, 2].contains(&g6) && is_prime_with_condition(3 + g) {
            return Some((3, 3 + g));
        }
        m = 5;
    }

    match g6 {
        0 => return step_divisible_by_6(g, m, n),
        2 => m += 5 - m % 6,
        4 => {
            let r = m % 6;
            m += if r <= 1 { 1 - r } else { 7 - r };
        }
        _ => return None,
    }

    while m <= n {
        if is_prime_with_condition(m) && is_prime_with_condition(m + g) {
            return Some((m, m + g));
        }

        m += 6;
    }

    None
}
