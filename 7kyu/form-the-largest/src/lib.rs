//! <https://www.codewars.com/kata/5a4ea304b3bfa89a9900008e/train/rust>

pub fn max_number(n: u32) -> u32 {
    fn to_digits(mut n: u32) -> ([u8; 10], usize) {
        let (mut digits, mut len) = ([0; 10], 0);
        while n != 0 {
            unsafe { *digits.get_unchecked_mut(len) = (n % 10) as u8 };
            n /= 10;
            len += 1;
        }
        if len > digits.len() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        (digits, len)
    }

    let (mut digits, len) = to_digits(n);
    digits[..len].sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    digits[..len].iter().fold(0, |acc, &d| 10 * acc + d as u32)
}
