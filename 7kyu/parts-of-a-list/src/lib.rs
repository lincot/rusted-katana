//! <https://www.codewars.com/kata/56f3a1e899b386da78000732/train/rust>

use my_prelude::prelude::*;

pub fn part_list(arr: Vec<&str>) -> String {
    match arr.len() {
        0 => return String::new(),
        1 => {
            let mut res = Vec::with_capacity(2 + arr[0].len());
            unsafe {
                res.push_unchecked(b'(');
                res.extend_from_slice_unchecked(arr[0].as_bytes());
                res.push_unchecked(b')');
                return String::from_utf8_unchecked(res);
            }
        }
        _ => {}
    }

    let cap = arr.iter().map(|s| s.len()).sum::<usize>() + arr.len() - 1;
    let mut joined = Vec::with_capacity(cap);
    let mut comma_poses = Vec::with_capacity(arr.len() - 1);

    for s in arr[..arr.len() - 1].iter() {
        unsafe {
            joined.extend_from_slice_unchecked(s.as_bytes());
            comma_poses.push_unchecked(joined.len());
            joined.push_unchecked(b' ');
        }
    }

    unsafe { joined.extend_from_slice_unchecked(arr[arr.len() - 1].as_bytes()) };

    let mut res = Vec::with_capacity(comma_poses.len() * (joined.len() + 3));

    for comma_pos in comma_poses {
        unsafe {
            res.push_unchecked(b'(');
            res.extend_from_slice_unchecked(joined.get_unchecked(..comma_pos));
            res.push_unchecked(b',');
            res.extend_from_slice_unchecked(joined.get_unchecked(comma_pos..));
            res.push_unchecked(b')');
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
