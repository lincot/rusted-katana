//! <https://www.codewars.com/kata/6498aa0daff4420024ce2c88/train/rust>

use core::cmp::Reverse;
use unchecked_std::prelude::*;

pub fn get_in_line(arr: &[i32]) -> i32 {
    let mut known_guest_count = 0;
    let mut members_count = 0;
    let mut decoy_positions = Vec::with_capacity(arr.len());
    let mut my_position = 0;

    for (i, &x) in arr.iter().enumerate().rev() {
        match x {
            0 => my_position = i + known_guest_count + members_count,
            1 => known_guest_count += 1,
            2 => members_count += 1,
            3 => unsafe { decoy_positions.push_unchecked(i + known_guest_count + members_count) },
            _ => {}
        }
    }

    for k in 0..known_guest_count {
        let k = k + 1;
        let mid = k + (arr.len() - k) / 2;
        let r = (arr.len() - k) % 2;
        let target_pos = if my_position <= mid {
            my_position + 2 * (mid - my_position) - 1 + r
        } else {
            my_position - 2 * (my_position - mid) - 1 + r
        };
        if decoy_positions
            .binary_search_by_key(&Reverse(target_pos), |&x| Reverse(x))
            .is_err()
            && target_pos >= known_guest_count
        {
            my_position = target_pos;
        }
    }

    (my_position + 1) as _
}
