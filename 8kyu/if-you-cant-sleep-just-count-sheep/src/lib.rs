//! <https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/train/rust>

use digital::{CountDigitsBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

const SHEEP: &str = " sheep...";

pub fn count_sheep(n: u32) -> String {
    #[expect(clippy::absurd_extreme_comparisons)]
    {
        assert!(n <= MAX_N);
    }
    let mut res = String::with_capacity(get_capacity(n));
    for sheep in 1..n + 1 {
        unsafe {
            res.write_num_unchecked(sheep, 10, false, false);
            res.push_str_unchecked(SHEEP);
        }
    }
    res
}

/// equals to `(1..=n).map(|x| x.to_string().len()).sum::<usize>() as u32`
fn count_digits_up_to(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    let log10 = n.count_digits_base10() - 1;
    let mut t = 1;
    for _ in 0..log10 {
        t = 10 * t + 1;
    }
    (log10 + 1) * n + (log10 + 1) - t
}

#[allow(dead_code)]
const MAX_N_64: u32 = u32::MAX;
#[allow(dead_code)]
const MAX_N_32: u32 = 125_477_486;
#[allow(dead_code)]
const MAX_N_16: u32 = 2605;

#[cfg(target_pointer_width = "64")]
const MAX_N: u32 = MAX_N_64;
#[cfg(target_pointer_width = "32")]
const MAX_N: u32 = MAX_N_32;
#[cfg(target_pointer_width = "16")]
const MAX_N: u32 = MAX_N_16;

fn get_capacity(n: u32) -> usize {
    count_digits_up_to(n as usize) + SHEEP.len() * n as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_n() {
        assert!(i64::try_from(get_capacity(MAX_N_64 as _)).is_ok());
        assert_eq!(MAX_N_64, u32::MAX);

        assert!(i32::try_from(get_capacity(MAX_N_32 as _)).is_ok());
        assert!(i32::try_from(get_capacity((MAX_N_32 + 1) as _)).is_err());

        assert!(i16::try_from(get_capacity(MAX_N_16 as _)).is_ok());
        assert!(i16::try_from(get_capacity((MAX_N_16 + 1) as _)).is_err());
    }
}
