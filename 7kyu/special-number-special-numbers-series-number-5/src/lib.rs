//! <https://www.codewars.com/kata/5a55f04be6be383a50000187/train/rust>

pub fn special_number(mut n: u64) -> String {
    while n >= 10 {
        if SPECIAL_100[(n % 100) as usize] {
            return "NOT!!".into();
        }
        n /= 100;
    }
    if n > 5 {
        "NOT!!".into()
    } else {
        "Special!!".into()
    }
}

const SPECIAL_100: [bool; 100] = [
    false, false, false, false, false, false, true, true, true, true, false, false, false, false,
    false, false, true, true, true, true, false, false, false, false, false, false, true, true,
    true, true, false, false, false, false, false, false, true, true, true, true, false, false,
    false, false, false, false, true, true, true, true, false, false, false, false, false, false,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true,
];
