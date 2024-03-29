//! <https://www.codewars.com/kata/57f6ad55cca6e045d2000627/train/rust>

pub fn square_or_square_root(arr: &[u32]) -> Vec<u32> {
    arr.iter()
        .map(|&x| perfect_sqrt(x).unwrap_or_else(|| x.wrapping_mul(x)))
        .collect()
}

fn perfect_sqrt(n: u32) -> Option<u32> {
    if [0, 1, 4, 9].contains(&(n & 0xf)) {
        let s = unsafe { (n as f64).sqrt().to_int_unchecked() };
        if s * s == n {
            Some(s)
        } else {
            None
        }
    } else {
        None
    }
}
