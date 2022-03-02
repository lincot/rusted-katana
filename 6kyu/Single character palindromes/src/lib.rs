//! <https://www.codewars.com/kata/5a2c22271f7f709eaa0005d3/train/rust>

pub fn solve(s: &str) -> String {
    let chars: Vec<_> = s.chars().collect();

    if chars.len() <= 1 {
        return "OK".into();
    }

    let mut i = 0;
    let mut j = chars.len() - 1;
    while i < j {
        if unsafe { chars.get_unchecked(i) != chars.get_unchecked(j) } {
            let mut i_ = i + 1;
            let mut j_ = j;
            let mut broke = false;
            while i_ < j_ {
                if unsafe { chars.get_unchecked(i_) != chars.get_unchecked(j_) } {
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
                if unsafe { chars.get_unchecked(i) != chars.get_unchecked(j) } {
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
