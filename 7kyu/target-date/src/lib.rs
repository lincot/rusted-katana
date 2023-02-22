//! <https://www.codewars.com/kata/569218bc919ccba77000000b/train/rust>

#![no_std]
#![feature(core_intrinsics)]

extern crate alloc;
use alloc::string::{String, ToString};
use chrono::{naive::NaiveDate, Duration};
use core::intrinsics::{fmaf64, logf64};

fn log(a: f64, b: f64) -> f64 {
    unsafe { logf64(a) / logf64(b) }
}

pub fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
    let p = unsafe { fmaf64(p as _, 1. / 100. / 360., 1.) };
    let days = log(a as f64 / a0 as f64, p) as _;
    (unsafe { NaiveDate::from_ymd_opt(2016, 1, 2).unwrap_unchecked() } + Duration::days(days))
        .to_string()
}
