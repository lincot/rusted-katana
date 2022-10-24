//! <https://www.codewars.com/kata/57873ab5e55533a2890000c7/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn time_correct(time_str: &str) -> Option<String> {
    let (hours, rest) = time_str.split_once(':')?;
    let (minutes, seconds) = rest.split_once(':')?;

    let mut hours: u8 = hours.parse().ok()?;
    let mut minutes: u8 = minutes.parse().ok()?;
    let mut seconds: u8 = seconds.parse().ok()?;

    minutes += seconds / 60;
    seconds %= 60;
    hours += minutes / 60;
    minutes %= 60;
    hours %= 24;

    let mut res = String::with_capacity("00:00:00".len());
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
        res.push_unchecked(':');
        if seconds < 10 {
            res.push_unchecked('0');
        }
        res.write_num_unchecked(seconds);
    }
    Some(res)
}
