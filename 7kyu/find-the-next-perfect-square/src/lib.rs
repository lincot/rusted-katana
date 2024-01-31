//! <https://www.codewars.com/kata/56269eb78ad2e4ced1000013/train/rust>

pub fn find_next_square(sq: u64) -> Option<u64> {
    perfect_sqrt(sq).map(|x| (x + 1).pow(2))
}

fn perfect_sqrt(n: u64) -> Option<u64> {
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
