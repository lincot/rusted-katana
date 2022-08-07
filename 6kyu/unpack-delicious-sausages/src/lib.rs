//! <https://www.codewars.com/kata/5fa6d9e9454977000fb0c1f8/train/rust>

use my_prelude::prelude::*;

pub fn unpack_sausages(truck: Vec<Vec<&str>>) -> String {
    // arbitrary
    let cap = 8 * truck.len();
    let mut sausages = Vec::with_capacity(cap);

    let mut undamaged = 0;
    for package in &truck {
        for package in package {
            let mut chars = package.chars();

            if chars.next() != Some('[') {
                continue;
            }

            let sausage = if let Some(first) = chars.next() {
                first
            } else {
                continue;
            };
            if (0..3).any(|_| chars.next() != Some(sausage)) {
                continue;
            }

            if chars.next() != Some(']') {
                continue;
            }

            undamaged += 1;
            if undamaged % 5 != 0 {
                sausages.push(sausage);
            }
        }
    }

    // worst case
    let cap = 20 * sausages.len();
    let mut res = String::with_capacity(cap);

    let mut sausages = sausages.into_iter();

    if let Some(sausage) = sausages.next() {
        unsafe { res.push_unchecked(sausage) };
        for _ in 0..3 {
            unsafe { res.push_unchecked(' ') };
            unsafe { res.push_unchecked(sausage) };
        }
    }
    for sausage in sausages {
        for _ in 0..4 {
            unsafe { res.push_unchecked(' ') };
            unsafe { res.push_unchecked(sausage) };
        }
    }

    res
}
