//! <https://www.codewars.com/kata/5b049d57de4c7f6a6c0001d7/train/rust>

use unchecked::{ExtendFromSliceUnchecked, PushUnchecked};

pub fn apparently(string: &str) -> String {
    let string = string.as_bytes();

    let mut res = Vec::with_capacity(4 * string.len() + string.len() * 2 / 3);

    if string.starts_with(b"and") || string.starts_with(b"but") {
        match string.len() {
            3 => unsafe {
                res.extend_from_slice_unchecked(string);
                res.extend_from_slice_unchecked(b" apparently");
                return String::from_utf8_unchecked(res);
            },
            4 => unsafe {
                res.extend_from_slice_unchecked(string);
                return String::from_utf8_unchecked(res);
            },
            _ => unsafe {
                if string[3] == b' '
                    && !(string[4..].starts_with(b"apparently")
                        && [None, Some(&b' ')].contains(&string.get(4 + b"apparently".len())))
                {
                    res.extend_from_slice_unchecked(&string[..4]);
                    res.extend_from_slice_unchecked(b"apparently ");
                    res.push_unchecked(string[4]);
                } else {
                    res.extend_from_slice_unchecked(&string[..5]);
                }
            },
        }
    } else {
        unsafe { res.extend_from_slice_unchecked(&string[..string.len().min(5)]) };
    }

    for i in 5..string.len() {
        unsafe {
            if [b" and ", b" but "].contains(&string[i - 5..i].try_into().unwrap())
                && !(string[i..].starts_with(b"apparently")
                    && [None, Some(&b' ')].contains(&string.get(i + b"apparently".len())))
            {
                res.extend_from_slice_unchecked(b"apparently ");
            }
            res.push_unchecked(string[i]);
        }
    }

    if string.ends_with(b" and") || string.ends_with(b" but") {
        unsafe { res.extend_from_slice_unchecked(b" apparently") };
    }

    unsafe { String::from_utf8_unchecked(res) }
}
