//! <https://www.codewars.com/kata/569218bc919ccba77000000b/train/rust>

use chrono::{naive::NaiveDate, Duration};

pub fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
    let p = (p as f64).mul_add(1. / 100. / 360., 1.);
    let days = (a as f64 / a0 as f64).log(p) as i64;
    format!("{}", NaiveDate::from_ymd(2016, 1, 2) + Duration::days(days))
}
