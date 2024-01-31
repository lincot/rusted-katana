//! <https://www.codewars.com/kata/604287495a72ae00131685c7/train/rust>

use digital::WriteNumUnchecked;

pub fn doubleton(mut num: u32) -> u32 {
    unsafe fn to_digits(n: u32) -> heapless::Vec<u8, { "1000000".len() }> {
        let mut digits = heapless::Vec::new();
        digits.write_num_unchecked(n, 10, false, true);
        digits
    }

    fn from_digits(digits: &[u8]) -> u32 {
        digits.iter().fold(0, |acc, &d| 10 * acc + d as u32)
    }

    if !(10..=1_000_000).contains(&num) {
        return 10;
    }

    num += 1;

    let mut digits = unsafe { to_digits(num) };

    let mut d0_poses = heapless::Vec::<usize, 6>::new();
    let mut d0 = 10;
    let mut first_d1_pos = usize::MAX;
    let mut d1 = 10;
    let mut d2_pos = 0;
    let mut d2 = 10;

    for (i, &d) in digits.iter().enumerate() {
        if d == d0 {
            unsafe { d0_poses.push_unchecked(i) };
            continue;
        }

        if d == d1 {
            if first_d1_pos == usize::MAX {
                first_d1_pos = i;
            }
            continue;
        }

        if d1 != 10 {
            d2 = d;
            d2_pos = i;
            break;
        }

        if d > d0 {
            d1 = d;
            if first_d1_pos == usize::MAX {
                first_d1_pos = i;
            }
        } else {
            d1 = d0;
            d0 = d;
            if d0_poses.is_empty() {
                unsafe { d0_poses.push_unchecked(i) };
            } else {
                if first_d1_pos == usize::MAX {
                    first_d1_pos = d0_poses[0];
                }
                d0_poses[0] = i;
                unsafe { d0_poses.set_len(1) };
            }
        }
    }

    if d1 == 10 {
        let len = digits.len();
        unsafe { *digits.get_unchecked_mut(len - 1) += 1 };
        return from_digits(&digits);
    }

    if d2 == 10 {
        return num;
    }

    if d2 < d0 {
        for d in unsafe { digits.get_unchecked_mut(d2_pos..) } {
            *d = d0;
        }
        return from_digits(&digits);
    } else if d2 < d1 {
        unsafe { *digits.get_unchecked_mut(d2_pos) = d1 };

        for d in unsafe { digits.get_unchecked_mut(d2_pos + 1..) } {
            *d = d0;
        }
        return from_digits(&digits);
    }

    let last_d0_pos = *unsafe { d0_poses.get_unchecked(d0_poses.len() - 1) };
    if first_d1_pos > last_d0_pos {
        unsafe { *digits.get_unchecked_mut(first_d1_pos) += 1 };
        for d in unsafe { digits.get_unchecked_mut(first_d1_pos + 1..) } {
            *d = d0;
        }
        return from_digits(&digits);
    }

    if d0_poses.len() == 1 {
        if d0 + 1 == d1 {
            unsafe { *digits.get_unchecked_mut(d0_poses[0]) = d1 };
            for d in unsafe { digits.get_unchecked_mut(d0_poses[0] + 1..) } {
                *d = 0;
            }
        } else {
            for d in unsafe { digits.get_unchecked_mut(d0_poses[0]..) } {
                *d = d0 + 1;
            }
        }

        return from_digits(&digits);
    }

    unsafe { *digits.get_unchecked_mut(last_d0_pos) = d1 };

    for d in unsafe { digits.get_unchecked_mut(last_d0_pos + 1..) } {
        *d = d0;
    }

    from_digits(&digits)
}
