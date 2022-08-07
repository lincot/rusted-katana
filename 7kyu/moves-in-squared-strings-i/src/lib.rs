//! <https://www.codewars.com/kata/56dbe0e313c2f63be4000b25/train/rust>

use my_prelude::prelude::*;

pub fn hor_mirror(s: String) -> String {
    let mut lines = s.rsplit('\n');
    let mut res = String::with_capacity(s.len());

    if let Some(line) = lines.next() {
        unsafe { res.push_str_unchecked(line) };
    }
    for line in lines {
        unsafe {
            res.push_unchecked('\n');
            res.push_str_unchecked(line);
        }
    }

    res
}

pub fn vert_mirror(s: String) -> String {
    let mut lines = s.split('\n');
    let mut res = String::with_capacity(s.len());

    if let Some(line) = lines.next() {
        unsafe { res.extend_unchecked(line.chars().rev()) };
    }
    for line in lines {
        unsafe {
            res.push_unchecked('\n');
            res.extend_unchecked(line.chars().rev());
        }
    }

    res
}

pub fn oper(f: fn(String) -> String, s: String) -> String {
    f(s)
}
