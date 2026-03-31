//! <https://www.codewars.com/kata/588e27b7d1140d31cb000060/train/rust>

use core::hint::unreachable_unchecked;

use unchecked_std::prelude::*;

pub fn generate_pairs(n: u32) -> Vec<(u32, u32)> {
    assert!(n <= MAX_N);
    let cap = get_capacity(n);
    let mut res = Vec::with_capacity(cap);
    for a in 0..=n {
        for b in a..=n {
            unsafe { res.push_unchecked((a, b)) };
        }
    }
    if res.len() != cap {
        unsafe { unreachable_unchecked() };
    }
    res
}

const fn get_capacity(n: u32) -> usize {
    (n as usize + 1) * (n as usize + 2) / 2
}

#[cfg(any(target_pointer_width = "64", test))]
const MAX_N_64: u32 = 1_518_500_248;
#[cfg(any(target_pointer_width = "32", test))]
const MAX_N_32: u32 = 23168;
#[cfg(any(target_pointer_width = "16", test))]
const MAX_N_16: u32 = 89;

#[cfg(target_pointer_width = "64")]
const MAX_N: u32 = MAX_N_64;
#[cfg(target_pointer_width = "32")]
const MAX_N: u32 = MAX_N_32;
#[cfg(target_pointer_width = "16")]
const MAX_N: u32 = MAX_N_16;

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::*;

    #[test]
    fn test_max_diff() {
        const MAX_LEN_64: u64 = i64::MAX as u64 / size_of::<(u32, u32)>() as u64;
        const MAX_LEN_32: u32 = i32::MAX as u32 / size_of::<(u32, u32)>() as u32;
        const MAX_LEN_16: u16 = i16::MAX as u16 / size_of::<(u32, u32)>() as u16;

        assert!(u64::try_from(get_capacity(MAX_N_64 as _)).unwrap() <= MAX_LEN_64);
        assert!(u64::try_from(get_capacity((MAX_N_64 + 1) as _)).unwrap() > MAX_LEN_64);

        assert!(u32::try_from(get_capacity(MAX_N_32 as _)).unwrap() <= MAX_LEN_32);
        assert!(u32::try_from(get_capacity((MAX_N_32 + 1) as _)).unwrap() > MAX_LEN_32);

        assert!(u16::try_from(get_capacity(MAX_N_16 as _)).unwrap() <= MAX_LEN_16);
        assert!(u16::try_from(get_capacity((MAX_N_16 + 1) as _)).unwrap() > MAX_LEN_16);
    }
}
