//! <https://www.codewars.com/kata/569df0bc5565b243d500002b/train/rust>

use digital::NumToString;

pub fn find_us(n1: u32, n2: u32, k: u32, prime_factors: &[u32], digits: &[u32]) -> Vec<u32> {
    let step = prime_factors.iter().product::<u32>();
    let mut res = Vec::new();
    let mut x = n1 + step - n1 % step;
    while x <= (n1 + k * n2) {
        if digits.iter().all(|&d| {
            x.to_heapless_string(true, true)
                .as_bytes()
                .contains(&(d as u8))
        }) {
            res.push(x);
        }

        x += step;
    }
    res
}
