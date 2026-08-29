//! <https://www.codewars.com/kata/647518391e258e80eedf6e06/train/rust>

use unchecked_std::prelude::*;

pub fn create_two_sets_of_equal_sum(n: u32) -> [Vec<u32>; 2] {
    if !matches!(n % 4, 0 | 3) {
        return [Vec::new(), Vec::new()];
    }

    if n % 2 == 1 {
        return [
            (0..(n + 1) / 2).map(|i| i + 1 + (i / 2 * 2)).collect(),
            (0..n / 2).map(|i| i + 3 + (i / 2 * 2)).collect(),
        ];
    }

    let mut first = Vec::with_capacity((n / 2) as usize);
    unsafe {
        for i in 1..n / 4 + 1 {
            first.push_unchecked(i);
        }
        for i in n - n / 4 + 1..n + 1 {
            first.push_unchecked(i);
        }
    }
    [first, (n / 4 + 1..n - n / 4 + 1).collect()]
}
