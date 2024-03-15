//! <https://www.codewars.com/kata/59321f29a010d5aa80000066/train/rust>

use core::hint::unreachable_unchecked;

pub fn shortest_arrang(n: u32) -> Option<Vec<u32>> {
    if n.count_ones() < 2 {
        return None;
    }

    let mut num_groups = 2;
    loop {
        if num_groups == 0 {
            unsafe { unreachable_unchecked() };
        }

        if n % num_groups == num_groups / 2 {
            let start = n / num_groups - num_groups / 2 + 1;
            return Some((start..start + num_groups).rev().collect());
        }
        num_groups += 1;

        if n % num_groups == 0 {
            let start = n / num_groups - num_groups / 2;
            return Some((start..start + num_groups).rev().collect());
        }
        num_groups += 1;
    }
}
