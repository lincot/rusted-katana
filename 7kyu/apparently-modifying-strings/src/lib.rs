//! <https://www.codewars.com/kata/5b049d57de4c7f6a6c0001d7/train/rust>

use unchecked_std::prelude::*;

pub fn apparently(string: &str) -> String {
    let string = string.as_bytes();

    assert!(string.len() <= MAX_STRING_LEN);
    let mut res = Vec::with_capacity(get_capacity(string.len()));

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

#[allow(dead_code)]
const MAX_STRING_LEN_64: usize = 1_976_436_865_040_309_101;
#[allow(dead_code)]
const MAX_STRING_LEN_32: usize = 460_175_067;
#[allow(dead_code)]
const MAX_STRING_LEN_16: usize = 7021;

#[cfg(target_pointer_width = "64")]
const MAX_STRING_LEN: usize = MAX_STRING_LEN_64;
#[cfg(target_pointer_width = "32")]
const MAX_STRING_LEN: usize = MAX_STRING_LEN_32;
#[cfg(target_pointer_width = "16")]
const MAX_STRING_LEN: usize = MAX_STRING_LEN_16;

const fn get_capacity(string_len: usize) -> usize {
    4 * string_len + string_len * 2 / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_string_list_len() {
        assert!(isize::try_from(get_capacity(MAX_STRING_LEN)).is_ok());
        assert!(isize::try_from(get_capacity(MAX_STRING_LEN + 1)).is_err());

        assert!(i64::try_from(get_capacity(MAX_STRING_LEN_64)).is_ok());
        assert!(i64::try_from(get_capacity(MAX_STRING_LEN_64 + 1)).is_err());

        assert!(i32::try_from(get_capacity(MAX_STRING_LEN_32)).is_ok());
        assert!(i32::try_from(get_capacity(MAX_STRING_LEN_32 + 1)).is_err());

        assert!(i16::try_from(get_capacity(MAX_STRING_LEN_16)).is_ok());
        assert!(i16::try_from(get_capacity(MAX_STRING_LEN_16 + 1)).is_err());
    }
}
