//! <https://www.codewars.com/kata/5a4ea304b3bfa89a9900008e/train/rust>

pub fn max_number(mut n: u32) -> u32 {
    let mut digits = Vec::with_capacity(10);

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.sort_unstable_by_key(|&v| std::cmp::Reverse(v));

    digits.into_iter().fold(0, |acc, d| 10 * acc + d)
}
