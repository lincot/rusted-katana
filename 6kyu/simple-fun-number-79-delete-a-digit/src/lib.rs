//! <https://www.codewars.com/kata/5894318275f2c75695000146/train/rust>

pub fn delete_digit(n: u32) -> u32 {
    const POWERS_OF_10: [u64; 12] = {
        let mut res = [1; 12];
        let mut i = 1;
        while i < res.len() {
            res[i] = 10 * res[i - 1];
            i += 1;
        }
        res
    };

    let mut res = 0;
    let mut i = 0;
    let n = n as u64;
    let mut a = POWERS_OF_10[0];
    while a <= n {
        let b = POWERS_OF_10[i + 1];
        res = res.max(n % a + n / b * a);
        a = b;
        i += 1;
    }
    res as _
}
