//! <https://www.codewars.com/kata/63744cbed39ec3376c84ff4a/train/rust>

pub fn arrow_search(string: &str) -> i64 {
    let mut res = 0;
    let mut current_len = 0;
    let mut is_left_arrow = false;
    let mut is_double_arrow = false;
    let mut is_single_arrow = false;
    for b in string.bytes() {
        match b {
            b'<' => {
                if is_left_arrow {
                    res -= if is_double_arrow {
                        2 * current_len
                    } else {
                        current_len
                    };
                }
                is_left_arrow = true;
                is_single_arrow = false;
                is_double_arrow = false;
                current_len = 1;
            }
            b'>' => {
                if is_left_arrow {
                    is_left_arrow = false;
                } else {
                    current_len += 1;
                    res += if is_double_arrow {
                        2 * current_len
                    } else {
                        current_len
                    };
                }
                is_single_arrow = false;
                is_double_arrow = false;
                current_len = 0;
            }
            b'-' => {
                if is_double_arrow {
                    if is_left_arrow {
                        res -= 2 * current_len;
                        is_left_arrow = false;
                    }
                    current_len = 1;
                    is_double_arrow = false;
                } else {
                    current_len += 1;
                }
                is_single_arrow = true;
            }
            b'=' => {
                if is_single_arrow {
                    if is_left_arrow {
                        res -= current_len;
                        is_left_arrow = false;
                    }
                    current_len = 1;
                } else {
                    current_len += 1;
                }
                is_single_arrow = false;
                is_double_arrow = true;
            }
            _ => {
                if is_left_arrow {
                    res -= if is_double_arrow {
                        2 * current_len
                    } else {
                        current_len
                    };
                    is_left_arrow = false;
                }
                current_len = 0;
                is_single_arrow = false;
                is_double_arrow = false;
            }
        }
    }

    if is_left_arrow {
        res -= if is_double_arrow {
            2 * current_len
        } else {
            current_len
        };
    }

    res
}
