//! <https://www.codewars.com/kata/5a2c22271f7f709eaa0005d3/train/rust>

use unchecked_std::prelude::*;

pub fn solve(s: &str) -> String {
    if s.is_ascii() {
        return solve_arr(s.as_bytes());
    }

    let mut chars = Vec::with_capacity(s.len());
    for ch in s.chars() {
        unsafe { chars.push_unchecked(ch) };
    }
    solve_arr(&chars)
}

pub fn solve_arr<T: Eq>(s: &[T]) -> String {
    if s.len() <= 1 {
        return "OK".into();
    }

    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if unsafe { s.get_unchecked(i) != s.get_unchecked(j) } {
            let mut i_ = i + 1;
            let mut j_ = j;
            let mut broke = false;
            while i_ < j_ {
                if unsafe { s.get_unchecked(i_) != s.get_unchecked(j_) } {
                    broke = true;
                    break;
                }

                i_ += 1;
                j_ -= 1;
            }
            if !broke {
                return "remove one".into();
            }

            j -= 1;
            while i < j {
                if unsafe { s.get_unchecked(i) != s.get_unchecked(j) } {
                    return "not possible".into();
                }

                i += 1;
                j -= 1;
            }

            return "remove one".into();
        }

        i += 1;
        j -= 1;
    }

    "OK".into()
}
