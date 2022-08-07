//! <https://www.codewars.com/kata/604287495a72ae00131685c7/train/rust>

pub fn doubleton(mut num: u32) -> u32 {
    unsafe fn to_digits(mut n: u32) -> ([u8; 7], usize) {
        let (mut digits, mut len) = ([0; 7], 0);
        while n != 0 {
            *digits.get_unchecked_mut(len) = (n % 10) as u8;
            n /= 10;
            len += 1;
        }
        digits.get_unchecked_mut(..len).reverse();
        if len > digits.len() {
            core::hint::unreachable_unchecked();
        }
        (digits, len)
    }

    fn from_digits(digits: &[u8]) -> u32 {
        digits.iter().fold(0, |acc, &d| 10 * acc + d as u32)
    }

    if !(10..=1_000_000).contains(&num) {
        return 10;
    }

    num += 1;

    let (mut digits, digits_len) = unsafe { to_digits(num) };

    let mut d0_poses = [0; 6];
    let mut d0_poses_len = 0;
    let mut d0 = 10;
    let mut first_d1_pos = None;
    let mut d1 = 10;
    let mut d2_pos = 0;
    let mut d2 = 10;

    for (i, &d) in digits[..digits_len].iter().enumerate() {
        if d == d0 {
            unsafe { *d0_poses.get_unchecked_mut(d0_poses_len) = i };
            d0_poses_len += 1;
            continue;
        }

        if d == d1 {
            first_d1_pos = Some(first_d1_pos.unwrap_or(i));
            continue;
        }

        if d1 != 10 {
            d2 = d;
            d2_pos = i;
            break;
        }

        if d > d0 {
            d1 = d;
            first_d1_pos = Some(first_d1_pos.unwrap_or(i));
        } else {
            d1 = d0;
            d0 = d;
            if d0_poses_len > 0 {
                first_d1_pos = Some(first_d1_pos.unwrap_or(d0_poses[0]));
            }
            d0_poses[0] = i;
            d0_poses_len = 1;
        }
    }

    if d1 == 10 {
        unsafe { *digits.get_unchecked_mut(digits_len - 1) += 1 };
        return from_digits(&digits[..digits_len]);
    }

    if d2 == 10 {
        return num;
    }

    if d2 < d0 {
        for d in unsafe { digits.get_unchecked_mut(d2_pos..digits_len) } {
            *d = d0;
        }
        return from_digits(&digits[..digits_len]);
    } else if d2 < d1 {
        unsafe { *digits.get_unchecked_mut(d2_pos) = d1 };

        for d in unsafe { digits.get_unchecked_mut(d2_pos + 1..digits_len) } {
            *d = d0;
        }
        return from_digits(&digits[..digits_len]);
    }

    let first_d1_pos = unsafe { first_d1_pos.unwrap_unchecked() };
    let last_d0_pos = *unsafe { d0_poses.get_unchecked(d0_poses_len - 1) };
    if first_d1_pos > last_d0_pos {
        unsafe { *digits.get_unchecked_mut(first_d1_pos) += 1 };
        for d in unsafe { digits.get_unchecked_mut(first_d1_pos + 1..digits_len) } {
            *d = d0;
        }
        return from_digits(&digits[..digits_len]);
    }

    if d0_poses_len == 1 {
        let first_d0_pos = unsafe { *d0_poses.get_unchecked(0) };
        if d0 + 1 == d1 {
            unsafe { *digits.get_unchecked_mut(first_d0_pos) = d1 };
            for d in unsafe { digits.get_unchecked_mut(first_d0_pos + 1..digits_len) } {
                *d = 0;
            }
        } else {
            for d in unsafe { digits.get_unchecked_mut(first_d0_pos..digits_len) } {
                *d = d0 + 1;
            }
        }

        return from_digits(&digits[..digits_len]);
    }

    unsafe { *digits.get_unchecked_mut(last_d0_pos) = d1 };

    for d in unsafe { digits.get_unchecked_mut(last_d0_pos + 1..digits_len) } {
        *d = d0;
    }

    from_digits(&digits[..digits_len])
}
