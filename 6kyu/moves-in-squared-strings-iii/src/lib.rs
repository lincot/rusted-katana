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
    assert!(s.len() == n * n + n - 1);

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
    assert!(s.len() == n * n + n - 1);

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

    let mut res = String::with_capacity(2 * s.len() + 2);
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
    assert!(s.len() == n * n + n - 1);

    let mut res = Vec::with_capacity(2 * s.len() + 2);
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
