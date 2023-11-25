//! <https://www.codewars.com/kata/64fbfa3518692c2ed0ebbaa2/train/rust>

#![no_std]

pub fn diving_minigame(lst: &[i8]) -> bool {
    let mut breath = 10;
    for &x in lst {
        if x < 0 {
            if breath <= 2 {
                return false;
            }
            breath -= 2;
        } else {
            breath = (breath + 4).min(10);
        }
    }
    true
}
