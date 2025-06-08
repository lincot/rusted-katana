//! <https://www.codewars.com/kata/5fa6d9e9454977000fb0c1f8/train/rust>

use unchecked_std::prelude::*;

pub fn unpack_sausages(truck: Vec<Vec<&str>>) -> String {
    let packages_num = truck
        .iter()
        .fold(0usize, |acc, l| acc.checked_add(l.len()).unwrap());
    let cap = (packages_num - packages_num / 5).checked_mul(20).unwrap();
    let mut res = String::with_capacity(cap);

    let mut undamaged = 0;
    for package in &truck {
        for package in package {
            let mut chars = package.chars();

            if chars.next() != Some('[') {
                continue;
            }

            let Some(sausage) = chars.next() else {
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
                for _ in 0..4 {
                    unsafe {
                        res.push_unchecked(sausage);
                        res.push_unchecked(' ');
                    }
                }
            }
        }
    }

    res.pop();

    res
}
