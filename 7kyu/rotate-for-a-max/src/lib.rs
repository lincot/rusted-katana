//! <https://www.codewars.com/kata/56a4872cbb65f3a610000026/train/rust>

pub fn max_rot(n: u64) -> u64 {
    #[inline(never)]
    fn to_digits(mut n: u64) -> heapless::Vec<u8, 20> {
        let mut digits = heapless::Vec::new();
        // TODO: make a better conversion
        while n != 0 {
            unsafe { digits.push_unchecked((n % 10) as _) };
            n /= 10;
        }
        digits
    }

    fn from_digits(digits: &[u8]) -> u64 {
        digits.iter().rev().fold(0, |acc, &d| 10 * acc + d as u64)
    }

    let mut digits = to_digits(n);
    let mut max_digits = digits.clone();

    for end in (1..digits.len()).rev() {
        if end >= digits.len() {
            unsafe { core::hint::unreachable_unchecked() };
        }

        digits[..=end].rotate_right(1);

        if digits[end] != max_digits[end] {
            if digits[end] > max_digits[end] {
                max_digits[..=end].copy_from_slice(&digits[..=end]);
                continue;
            }
            break;
        }

        for i in (0..end).rev() {
            unsafe {
                if digits.get_unchecked(i) != max_digits.get_unchecked(i) {
                    if digits.get_unchecked(i) > max_digits.get_unchecked(i) {
                        max_digits
                            .get_unchecked_mut(..=i)
                            .copy_from_slice(digits.get_unchecked_mut(..=i));
                    }
                    break;
                }
            }
        }
    }

    from_digits(&max_digits)
}
