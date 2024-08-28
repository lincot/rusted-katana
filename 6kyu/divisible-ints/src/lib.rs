//! <https://www.codewars.com/kata/566859a83557837d9700001a/train/rust>

use digital::NumToString;

pub fn get_count(n: u64) -> u32 {
    let num = n.to_heapless_string(true, true);
    let num = num.as_bytes();
    let mut res = 0;
    for len in 1..num.len() {
        let mut sub: u64 = num[..len]
            .iter()
            .rev()
            .fold(0, |acc, &x| 10 * acc + x as u64);
        if sub != 0 && n % sub == 0 {
            res += 1;
        }
        for start in 0..num.len() - len {
            sub = (sub - num[start] as u64) / 10
                + unsafe {
                    POWERS_OF_10.get_unchecked(len) * *num.get_unchecked(start + len) as u64
                };
            if sub != 0 && n % sub == 0 {
                res += 1;
            }
        }
    }
    res
}

const POWERS_OF_10: [u64; 20] = {
    let mut res = [1; 20];
    let mut i = 2;
    while i < res.len() {
        res[i] = 10 * res[i - 1];
        i += 1;
    }
    res
};
