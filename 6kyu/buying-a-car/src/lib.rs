//! <https://www.codewars.com/kata/554a44516729e4d80b000012/train/rust>

#![no_std]

pub fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut diff = (new - old) as f64;
    let mut savings = 0;
    let mut months = 0;
    let mut perc = 1. - perc / 100.;
    while diff > savings as _ {
        diff *= perc;
        savings += saving;
        months += 1;
        if months % 2 == 1 {
            perc -= 0.005;
        }
    }
    (months, (savings as f64 - diff + 0.5) as _)
}
