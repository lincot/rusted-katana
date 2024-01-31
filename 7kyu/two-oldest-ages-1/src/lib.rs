//! <https://www.codewars.com/kata/511f11d355fe575d2c000001/train/rust>

pub fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
    let mut max0 = 0;
    let mut max1 = 0;

    for &age in ages {
        if age > max0 {
            max1 = max0;
            max0 = age;
        } else if age > max1 {
            max1 = age;
        }
    }

    [max1, max0]
}
