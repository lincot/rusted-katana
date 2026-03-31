//! <https://www.codewars.com/kata/588e2a1ad1140d31cb00008c/train/rust>

use unchecked_std::prelude::*;

pub fn generate_pairs(m: i16, n: i16) -> Vec<(i16, i16)> {
    let diff = if n >= m {
        (n as i32 - m as i32 + 1) as u32
    } else {
        0
    };
    assert!(diff <= MAX_DIFF);
    let mut res = Vec::with_capacity(get_capacity(diff as usize));
    if diff == 0 {
        return res;
    }
    let mut first = m;
    loop {
        let mut second = first;
        loop {
            unsafe { res.push_unchecked((first, second)) };
            if second == n {
                break;
            }
            second += 1;
        }
        if first == n {
            break;
        }
        first += 1;
    }
    res
}

#[cfg(any(target_pointer_width = "64", test))]
const MAX_DIFF_64: u32 = 2_147_483_647;
#[cfg(any(target_pointer_width = "32", test))]
const MAX_DIFF_32: u32 = 32_767;
#[cfg(any(target_pointer_width = "16", test))]
const MAX_DIFF_16: u32 = 127;

#[cfg(target_pointer_width = "64")]
const MAX_DIFF: u32 = MAX_DIFF_64;
#[cfg(target_pointer_width = "32")]
const MAX_DIFF: u32 = MAX_DIFF_32;
#[cfg(target_pointer_width = "16")]
const MAX_DIFF: u32 = MAX_DIFF_16;

const fn get_capacity(diff: usize) -> usize {
    (diff * diff + diff) / 2
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::*;

    #[test]
    fn test_max_diff() {
        const MAX_LEN_64: u64 = i64::MAX as u64 / size_of::<(i16, i16)>() as u64;
        const MAX_LEN_32: u32 = i32::MAX as u32 / size_of::<(i16, i16)>() as u32;
        const MAX_LEN_16: u16 = i16::MAX as u16 / size_of::<(i16, i16)>() as u16;

        assert!(u64::try_from(get_capacity(MAX_DIFF_64 as _)).unwrap() <= MAX_LEN_64);
        assert!(u64::try_from(get_capacity((MAX_DIFF_64 + 1) as _)).unwrap() > MAX_LEN_64);

        assert!(u32::try_from(get_capacity(MAX_DIFF_32 as _)).unwrap() <= MAX_LEN_32);
        assert!(u32::try_from(get_capacity((MAX_DIFF_32 + 1) as _)).unwrap() > MAX_LEN_32);

        assert!(u16::try_from(get_capacity(MAX_DIFF_16 as _)).unwrap() <= MAX_LEN_16);
        assert!(u16::try_from(get_capacity((MAX_DIFF_16 + 1) as _)).unwrap() > MAX_LEN_16);
    }
}
