//! <https://www.codewars.com/kata/56dbe7f113c2f63570000b86/train/rust>

use unchecked_std::prelude::*;

pub fn rot(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(s.bytes().rev().collect()) };
    }

    let mut res = String::with_capacity(s.len());
    for ch in s.chars().rev() {
        unsafe { res.push_unchecked(ch) };
    }
    res
}

pub fn selfie_and_rot(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(selfie_and_rot_bytes(s.as_bytes())) };
    }

    assert!(s.len() <= MAX_LEN);
    let mut res = String::with_capacity(get_capacity_selfie_and_rot(s.len()));
    let mut row_len = 0usize;
    let mut chars = s.chars();
    for ch in chars.by_ref() {
        if ch == '\n' {
            break;
        }
        unsafe { res.push_unchecked(ch) };
        row_len += 1;
    }
    if row_len == 0 {
        return String::new();
    }
    unsafe { res.as_mut_vec().push_many_unchecked(b'.', row_len) };

    for _ in 0..row_len - 1 {
        unsafe {
            res.push_unchecked('\n');
            for _ in 0..row_len {
                res.push_unchecked(chars.next().unwrap());
            }
            chars.next();
            res.as_mut_vec().push_many_unchecked(b'.', row_len);
        }
    }

    let mut chars = s.chars().rev();
    for _ in 0..row_len {
        unsafe {
            res.push_unchecked('\n');
            res.as_mut_vec().push_many_unchecked(b'.', row_len);

            for _ in 0..row_len {
                res.push_unchecked(chars.next().unwrap_unchecked());
            }
            chars.next();
        }
    }

    res
}

fn selfie_and_rot_bytes(s: &[u8]) -> Vec<u8> {
    let n = s.iter().position(|&b| b == b'\n').unwrap_or(s.len());
    assert!(n <= MAX_N && s.len() == get_len(n));
    let mut res = Vec::with_capacity(get_capacity_selfie_and_rot_bytes(s.len(), n));

    let mut start = 0;
    for _ in 0..n {
        unsafe {
            res.extend_from_slice_unchecked(s.get_unchecked(start..start + n));
            start += n + 1;
            res.push_many_unchecked(b'.', n);
            res.push_unchecked(b'\n');
        }
    }

    let mut i = n * (n + 1) - 2;
    for _ in 0..n {
        unsafe {
            res.push_many_unchecked(b'.', n);
            for _ in 0..n {
                res.push_unchecked(*s.get_unchecked(i));
                i = i.wrapping_sub(1);
            }
            res.push_unchecked(b'\n');
            i = i.wrapping_sub(1);
        }
    }

    res.pop();
    res
}

pub fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}

const fn get_len(n: usize) -> usize {
    n * n + n - 1
}

const fn get_capacity_selfie_and_rot(len: usize) -> usize {
    4 * len
}

const fn get_capacity_selfie_and_rot_bytes(len: usize, n: usize) -> usize {
    4 * len - 2 * n + 4
}

#[cfg(any(target_pointer_width = "64", test))]
const MAX_N_64: usize = 1_518_500_249;
#[cfg(any(target_pointer_width = "32", test))]
const MAX_N_32: usize = 23170;
#[cfg(any(target_pointer_width = "16", test))]
const MAX_N_16: usize = 90;

#[cfg(target_pointer_width = "64")]
const MAX_N: usize = MAX_N_64;
#[cfg(target_pointer_width = "32")]
const MAX_N: usize = MAX_N_32;
#[cfg(target_pointer_width = "16")]
const MAX_N: usize = MAX_N_16;

#[cfg(any(target_pointer_width = "64", test))]
const MAX_LEN_64: usize = 2_305_843_009_213_693_951;
#[cfg(any(target_pointer_width = "32", test))]
const MAX_LEN_32: usize = 536_870_911;
#[cfg(any(target_pointer_width = "16", test))]
const MAX_LEN_16: usize = 8191;

#[cfg(target_pointer_width = "64")]
const MAX_LEN: usize = MAX_LEN_64;
#[cfg(target_pointer_width = "32")]
const MAX_LEN: usize = MAX_LEN_32;
#[cfg(target_pointer_width = "16")]
const MAX_LEN: usize = MAX_LEN_16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_n() {
        assert!(i64::try_from(get_capacity_selfie_and_rot_bytes(
            get_len(MAX_N_64),
            MAX_N_64
        ))
        .is_ok());
        assert!(i64::try_from(get_capacity_selfie_and_rot_bytes(
            get_len(MAX_N_64 + 1),
            MAX_N_64 + 1
        ))
        .is_err());

        assert!(i32::try_from(get_capacity_selfie_and_rot_bytes(
            get_len(MAX_N_32),
            MAX_N_32
        ))
        .is_ok());
        assert!(i32::try_from(get_capacity_selfie_and_rot_bytes(
            get_len(MAX_N_32 + 1),
            MAX_N_32 + 1
        ))
        .is_err());

        assert!(i16::try_from(get_capacity_selfie_and_rot_bytes(
            get_len(MAX_N_16),
            MAX_N_16
        ))
        .is_ok());
        assert!(i16::try_from(get_capacity_selfie_and_rot_bytes(
            get_len(MAX_N_16 + 1),
            MAX_N_16 + 1
        ))
        .is_err());
    }

    #[test]
    fn test_max_len() {
        assert!(i64::try_from(get_capacity_selfie_and_rot(MAX_LEN_64)).is_ok());
        assert!(i64::try_from(get_capacity_selfie_and_rot(MAX_LEN_64 + 1,)).is_err());

        assert!(i32::try_from(get_capacity_selfie_and_rot(MAX_LEN_32)).is_ok());
        assert!(i32::try_from(get_capacity_selfie_and_rot(MAX_LEN_32 + 1)).is_err());

        assert!(i16::try_from(get_capacity_selfie_and_rot(MAX_LEN_16)).is_ok());
        assert!(i16::try_from(get_capacity_selfie_and_rot(MAX_LEN_16 + 1)).is_err());
    }
}
