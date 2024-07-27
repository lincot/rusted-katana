//! <https://www.codewars.com/kata/64e5e192f1294f96fd60a5ae/train/rust>

use unchecked_std::prelude::*;

pub fn block_pushing(lst: &[char], n: u32) -> Vec<char> {
    fn block_binary_search(lst: &[usize], i: usize, n: usize) -> usize {
        let mut left = 0;
        let mut right = lst.len();
        while left < right {
            let mid = (left + right) / 2;
            if unsafe { lst.get_unchecked(mid) } - i >= n + lst.len() - mid {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        lst.len() - left
    }

    if lst.is_empty() {
        return Vec::new();
    }

    let n = n as usize;
    let mut res = vec!['-'; lst.len()];
    let mut limit = res.len() - 1;
    let mut block_positions = Vec::with_capacity(lst.len());

    for (i, &x) in lst.iter().enumerate().rev() {
        if x == '#' {
            unsafe { block_positions.push_unchecked(i) };
        } else if x == '>' {
            let offset = block_binary_search(&block_positions, i, n);

            let pusher_target_pos = (i + n).min(limit - offset);
            if pusher_target_pos == limit - offset {
                limit = pusher_target_pos.wrapping_sub(1);
            }
            unsafe {
                *res.get_unchecked_mut(pusher_target_pos) = '>';
                res.get_unchecked_mut(pusher_target_pos + 1..pusher_target_pos + 1 + offset)
                    .fill('#');
                for &b in block_positions.get_unchecked(..block_positions.len() - offset) {
                    *res.get_unchecked_mut(b) = '#';
                }
            }
            block_positions.truncate(0);
        }
    }

    for b in block_positions {
        unsafe { *res.get_unchecked_mut(b) = '#' };
    }

    res
}
