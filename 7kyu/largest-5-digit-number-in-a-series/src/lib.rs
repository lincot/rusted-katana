//! <https://www.codewars.com/kata/51675d17e0c1bed195000001/train/rust>

use std::cmp::Ordering;

pub fn largest_five_digit_number(num: &str) -> u32 {
    let mut d0 = 0;
    let mut d1 = 0;
    let mut d2 = 0;
    let mut d3 = 0;
    let mut d4 = 0;

    let num: Box<_> = num.bytes().map(|b| b - b'0').collect();

    for i in 0..num.len() - 4 {
        if i >= num.len()
            || i + 1 >= num.len()
            || i + 2 >= num.len()
            || i + 3 >= num.len()
            || i + 4 >= num.len()
        {
            unsafe {
                core::hint::unreachable_unchecked();
            }
        }

        match num[i].cmp(&d0) {
            Ordering::Greater => {
                d0 = num[i];
                d1 = num[i + 1];
                d2 = num[i + 2];
                d3 = num[i + 3];
                d4 = num[i + 4];
            }
            Ordering::Equal => match num[i + 1].cmp(&d1) {
                Ordering::Greater => {
                    d1 = num[i + 1];
                    d2 = num[i + 2];
                    d3 = num[i + 3];
                    d4 = num[i + 4];
                }
                Ordering::Equal => match num[i + 2].cmp(&d2) {
                    Ordering::Greater => {
                        d2 = num[i + 2];
                        d3 = num[i + 3];
                        d4 = num[i + 4];
                    }
                    Ordering::Equal => match num[i + 3].cmp(&d3) {
                        Ordering::Greater => {
                            d3 = num[i + 3];
                            d4 = num[i + 4];
                        }
                        Ordering::Equal => {
                            if num[i + 4] > d4 {
                                d4 = num[i + 4];
                            }
                        }
                        Ordering::Less => {}
                    },
                    Ordering::Less => {}
                },
                Ordering::Less => {}
            },
            Ordering::Less => {}
        }
    }

    10000 * d0 as u32 + 1000 * d1 as u32 + 100 * d2 as u32 + 10 * d3 as u32 + d4 as u32
}
