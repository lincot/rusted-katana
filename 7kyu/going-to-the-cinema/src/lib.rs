//! <https://www.codewars.com/kata/562f91ff6a8b77dfe900006e/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::powif64;

pub fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let mut n = card / ticket;
    let mut a = ticket * n;
    let mut p = unsafe { powif64(perc, n) };
    let mut b = card as f64 + ticket as f64 * perc * (p - 1.) / (perc - 1.);

    while ((b + 0.99999) as i32) >= a {
        p *= perc;
        a += ticket;
        b += ticket as f64 * p;
        n += 1;
    }

    n
}
