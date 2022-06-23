//! <https://www.codewars.com/kata/5a55f04be6be383a50000187/train/rust>

pub fn special_number(mut n: u64) -> String {
    while n != 0 {
        if n % 10 > 5 {
            return "NOT!!".into();
        }

        n /= 10;
    }

    "Special!!".into()
}
