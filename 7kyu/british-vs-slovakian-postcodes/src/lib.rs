//! <https://www.codewars.com/kata/664e4f5b12b1b20182ea3e3a/train/rust>

pub fn which_postcode(s: &str) -> String {
    let not_valid = || "Not valid".into();

    let start = s.bytes().position(|b| b != b' ').unwrap_or(s.len());
    let s = unsafe { s.as_bytes().get_unchecked(start..) };
    if s.len() < 6 {
        return not_valid();
    }

    if s[0].is_ascii_digit()
        && s[1].is_ascii_digit()
        && s[2].is_ascii_digit()
        && s[3] == b' '
        && s[4].is_ascii_digit()
        && s[5].is_ascii_digit()
        && s[6..].iter().all(|&b| b == b' ')
    {
        "SK".into()
    } else {
        if !s[0].is_ascii_alphabetic() {
            return not_valid();
        }
        let second_seg_start = if s[1].is_ascii_alphabetic() {
            if !s[2].is_ascii_digit() {
                return not_valid();
            }
            if s[3].is_ascii_digit() {
                if s[4] != b' ' {
                    return not_valid();
                }
                5
            } else if s[3] == b' ' {
                4
            } else {
                return not_valid();
            }
        } else if s[1].is_ascii_digit() {
            if s[2].is_ascii_digit() {
                if s[3] != b' ' {
                    return not_valid();
                }
                4
            } else if s[2] == b' ' {
                3
            } else {
                return not_valid();
            }
        } else {
            return not_valid();
        };

        if second_seg_start + 2 < s.len()
            && s[second_seg_start].is_ascii_digit()
            && s[second_seg_start + 1].is_ascii_alphabetic()
            && s[second_seg_start + 2].is_ascii_alphabetic()
            && s[second_seg_start + 3..].iter().all(|&b| b == b' ')
        {
            "GB".into()
        } else {
            not_valid()
        }
    }
}
