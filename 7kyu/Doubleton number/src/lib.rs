//! <https://www.codewars.com/kata/604287495a72ae00131685c7/train/rust>

pub fn doubleton(mut num: u32) -> u32 {
    fn to_digits(mut n: u32) -> Vec<u8> {
        let mut digits = Vec::with_capacity(7);

        while n != 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }

        digits.reverse();
        digits
    }

    fn from_digits(digits: Vec<u8>) -> u32 {
        digits.into_iter().fold(0, |acc, d| 10 * acc + d as u32)
    }

    if num < 10 {
        return 10;
    }

    num += 1;

    let mut digits = to_digits(num);

    let mut d0_poses = Vec::with_capacity(6);
    let mut d0 = 10;
    let mut d1_poses = Vec::with_capacity(5);
    let mut d1 = 10;
    let mut d2_pos = 0;
    let mut d2 = 10;

    for (i, &d) in digits.iter().enumerate() {
        if d == d0 {
            d0_poses.push(i);
            continue;
        }

        if d == d1 {
            d1_poses.push(i);
            continue;
        }

        if d1 != 10 {
            d2 = d;
            d2_pos = i;
            break;
        }

        if d > d0 {
            d1 = d;
            d1_poses.push(i);
        } else {
            d1 = d0;
            d0 = d;
            d1_poses.extend(&d0_poses);
            d0_poses.clear();
            d0_poses.push(i);
        }
    }

    if d1 == 10 {
        let len = digits.len();
        unsafe {
            *digits.get_unchecked_mut(len - 1) += 1;
        }
        return from_digits(digits);
    }

    if d2 == 10 {
        return num;
    }

    if d2 < d0 {
        for d in digits.iter_mut().skip(d2_pos) {
            *d = d0;
        }
        return from_digits(digits);
    } else if d2 < d1 {
        unsafe {
            let at_d2_pos = digits.get_unchecked_mut(d2_pos);
            *at_d2_pos = d1;
        }

        for d in digits.iter_mut().skip(d2_pos + 1) {
            *d = d0;
        }
        return from_digits(digits);
    }

    let first_d1_pos = *unsafe { d1_poses.get_unchecked(0) };
    let last_d0_pos = *unsafe { d0_poses.get_unchecked(d0_poses.len() - 1) };
    if first_d1_pos > last_d0_pos {
        unsafe {
            let at_first_d1_pos = digits.get_unchecked_mut(first_d1_pos);
            *at_first_d1_pos += 1;
        }
        for d in digits.iter_mut().skip(first_d1_pos + 1) {
            *d = d0;
        }
        return from_digits(digits);
    }

    if d0_poses.len() == 1 {
        let first_d0_pos = unsafe { *d0_poses.get_unchecked(0) };
        if d0 + 1 == d1 {
            unsafe {
                let at_first_d0_pos = digits.get_unchecked_mut(first_d0_pos);
                *at_first_d0_pos = d1;
            }
            for d in digits.iter_mut().skip(first_d0_pos + 1) {
                *d = 0;
            }
        } else {
            for d in digits.iter_mut().skip(first_d0_pos) {
                *d = d0 + 1;
            }
        }

        return from_digits(digits);
    }

    unsafe {
        let at_last_d0_pos = digits.get_unchecked_mut(last_d0_pos);
        *at_last_d0_pos = d1;
    }
    for d in digits.iter_mut().skip(last_d0_pos + 1) {
        *d = d0;
    }

    from_digits(digits)
}
