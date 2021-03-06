//! <https://www.codewars.com/kata/56269eb78ad2e4ced1000013/train/rust>

pub fn find_next_square(sq: u64) -> Option<u64> {
    perfect_sqrt(sq).map(|x| (x + 1).pow(2))
}

fn perfect_sqrt(n: u64) -> Option<u64> {
    match n & 0xf {
        0 | 1 | 4 | 9 => {
            let s = (n as f64).sqrt() as _;

            if s * s == n {
                Some(s)
            } else {
                None
            }
        }
        _ => None,
    }
}
