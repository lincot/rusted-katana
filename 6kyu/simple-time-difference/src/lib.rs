//! <https://www.codewars.com/kata/5b76a34ff71e5de9db0000f2/train/rust>

use my_prelude::prelude::*;

fn parse_time(time: &str) -> u32 {
    assert_eq!(time.len(), 5);

    let hours: u8 = time[..2].parse().unwrap();
    let minutes: u8 = time[3..].parse().unwrap();

    60 * hours as u32 + minutes as u32
}

pub fn solve(arr: &[&str]) -> String {
    assert!(!arr.is_empty());

    let mut arr: Vec<_> = arr.iter().map(|s| parse_time(s)).collect();
    arr.sort_unstable();

    let max_diff = arr
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .max()
        .unwrap_or_default()
        .max(24 * 60 + arr[0] - arr[arr.len() - 1]);

    let hours = (max_diff - 1) / 60;
    let minutes = (max_diff - 1) % 60;
    let mut res = String::with_capacity(3 + 1 + 2);
    unsafe {
        if hours < 10 {
            res.push_unchecked('0');
        }
        res.write_num_unchecked(hours);
        res.push_unchecked(':');
        if minutes < 10 {
            res.push_unchecked('0');
        }
        res.write_num_unchecked(minutes);
    }
    res
}
