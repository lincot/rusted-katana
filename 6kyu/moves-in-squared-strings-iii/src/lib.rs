//! <https://www.codewars.com/kata/56dbeec613c2f63be4000be6/train/rust>

use unchecked_std::prelude::*;

pub fn diag_1_sym(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(diag_1_sym_bytes(s.as_bytes())) };
    }

    let mut strs: Box<[_]> = s
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|s| unsafe { core::str::from_utf8_unchecked(s) }.chars())
        .collect();
    let n = strs.len();

    let mut res = String::with_capacity(s.len() + 1);
    for _ in 0..n {
        unsafe {
            for i in 0..n {
                res.push_unchecked(strs[i].next().unwrap());
            }
            res.push_unchecked('\n');
        }
    }
    res.pop();
    res
}

fn diag_1_sym_bytes(s: &[u8]) -> Vec<u8> {
    let n = s.iter().position(|&b| b == b'\n').unwrap_or(s.len());
    assert!(n <= MAX_N && s.len() == get_len(n));

    let mut res = Vec::with_capacity(s.len() + 1);
    for row in 0..n {
        unsafe {
            for i in 0..n {
                res.push_unchecked(*s.get_unchecked(i * (n + 1) + row));
            }
            res.push_unchecked(b'\n');
        }
    }
    res.pop();
    res
}

pub fn rot_90_clock(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(rot_90_clock_bytes(s.as_bytes())) };
    }

    let mut strs: Box<[_]> = s.split('\n').map(|s| s.chars()).collect();
    let n = strs.len();

    let mut res = String::with_capacity(s.len() + 1);
    for _ in 0..n {
        unsafe {
            for i in 0..n {
                res.push_unchecked(strs[strs.len() - 1 - i].next().unwrap());
            }
            res.push_unchecked('\n');
        }
    }
    res.pop();
    res
}

fn rot_90_clock_bytes(s: &[u8]) -> Vec<u8> {
    let n = s.iter().position(|&b| b == b'\n').unwrap_or(s.len());
    assert!(n <= MAX_N && s.len() == get_len(n));

    let mut res = Vec::with_capacity(s.len() + 1);
    for row in 0..n {
        unsafe {
            for i in 0..n {
                res.push_unchecked(*s.get_unchecked((n - 1 - i) * (n + 1) + row));
            }
            res.push_unchecked(b'\n');
        }
    }
    res.pop();
    res
}

pub fn selfie_and_diag1(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(selfie_and_diag1_bytes(s.as_bytes())) };
    }

    let mut strs: Box<[_]> = s
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|s| {
            let s = unsafe { core::str::from_utf8_unchecked(s) };
            (s, s.chars())
        })
        .collect();
    let n = strs.len();

    let mut res = String::with_capacity((2 * s.len()).checked_add(2).unwrap());
    for row in 0..n {
        unsafe {
            res.push_str_unchecked(strs[row].0);
            res.push_unchecked('|');
            for i in 0..n {
                res.push_unchecked(strs[i].1.next().unwrap());
            }
            res.push_unchecked('\n');
        }
    }
    res.pop();
    res
}

fn selfie_and_diag1_bytes(s: &[u8]) -> Vec<u8> {
    let n = s.iter().position(|&b| b == b'\n').unwrap_or(s.len());
    assert!(n <= MAX_N && s.len() == get_len(n));

    let mut res = Vec::with_capacity(get_capacity_selfie_and_diag1_bytes(s.len()));
    for row in 0..n {
        unsafe {
            res.extend_from_slice_unchecked(s.get_unchecked(row * (n + 1)..row * (n + 1) + n));
            res.push_unchecked(b'|');
            for i in 0..n {
                res.push_unchecked(*s.get_unchecked(i * (n + 1) + row));
            }
            res.push_unchecked(b'\n');
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

const fn get_capacity_selfie_and_diag1_bytes(len: usize) -> usize {
    2 * len + 2
}

#[cfg(any(target_pointer_width = "64", test))]
const MAX_N_64: usize = 3_037_000_499;
#[cfg(any(target_pointer_width = "32", test))]
const MAX_N_32: usize = 46340;
#[cfg(any(target_pointer_width = "16", test))]
const MAX_N_16: usize = 180;

#[cfg(target_pointer_width = "64")]
const MAX_N: usize = MAX_N_64;
#[cfg(target_pointer_width = "32")]
const MAX_N: usize = MAX_N_32;
#[cfg(target_pointer_width = "16")]
const MAX_N: usize = MAX_N_16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_n() {
        assert!(i64::try_from(get_len(MAX_N_64 as _)).is_ok());
        assert!(i64::try_from(get_len((MAX_N_64 + 1) as _)).is_err());
        assert!(u64::try_from(get_capacity_selfie_and_diag1_bytes(get_len(MAX_N_64))).is_ok());

        assert!(i32::try_from(get_len(MAX_N_32 as _)).is_ok());
        assert!(i32::try_from(get_len((MAX_N_32 + 1) as _)).is_err());
        assert!(u32::try_from(get_capacity_selfie_and_diag1_bytes(get_len(MAX_N_32))).is_ok());

        assert!(i16::try_from(get_len(MAX_N_16 as _)).is_ok());
        assert!(i16::try_from(get_len((MAX_N_16 + 1) as _)).is_err());
        assert!(u16::try_from(get_capacity_selfie_and_diag1_bytes(get_len(MAX_N_16))).is_ok());
    }
}
