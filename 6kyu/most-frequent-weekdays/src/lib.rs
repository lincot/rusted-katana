//! <https://www.codewars.com/kata/56eb16655250549e4b0013f4/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec, vec::Vec};

pub fn most_frequent_days(year: i32) -> Vec<String> {
    const fn format_weekday(day: u8) -> &'static str {
        match day {
            1 => "Tuesday",
            2 => "Wednesday",
            3 => "Thursday",
            4 => "Friday",
            5 => "Saturday",
            6 => "Sunday",
            _ => "Monday",
        }
    }

    assert!(year >= 0);

    let day = ((year + year / 4 - year / 100 + year / 400 + 5) % 7) as u8;

    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        if day < 6 {
            vec![format_weekday(day).into(), format_weekday(day + 1).into()]
        } else {
            vec!["Monday".into(), "Sunday".into()]
        }
    } else {
        vec![format_weekday(day + 1).into()]
    }
}
