//! <https://www.codewars.com/kata/56f3a1e899b386da78000732/train/rust>

use unchecked_std::prelude::*;

pub fn part_list(arr: Vec<&str>) -> String {
    match arr.len() {
        0 => return String::new(),
        1 => {
            let mut res = String::with_capacity(2 + arr[0].len());
            unsafe {
                res.push_unchecked('(');
                res.push_str_unchecked(arr[0]);
                res.push_unchecked(')');
                return res;
            }
        }
        _ => {}
    }

    let mut cap = arr.len() - 1;
    for s in &arr {
        cap = cap.checked_add(s.len()).unwrap();
    }

    let mut joined = String::with_capacity(cap);
    let mut comma_poses = Vec::with_capacity(arr.len() - 1);
    for s in &arr[..arr.len() - 1] {
        unsafe {
            joined.push_str_unchecked(s);
            comma_poses.push_unchecked(joined.len());
            joined.push_unchecked(' ');
        }
    }
    unsafe { joined.push_str_unchecked(arr[arr.len() - 1]) };

    let mut res = String::with_capacity(comma_poses.len().checked_mul(joined.len() + 3).unwrap());
    for comma_pos in comma_poses {
        unsafe {
            res.push_unchecked('(');
            res.push_str_unchecked(joined.get_unchecked(..comma_pos));
            res.push_unchecked(',');
            res.push_str_unchecked(joined.get_unchecked(comma_pos..));
            res.push_unchecked(')');
        }
    }
    res
}
