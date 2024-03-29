//! <https://www.codewars.com/kata/557e8a141ca1f4caa70000a6/train/rust>

pub fn is_triangle_number(n: u64) -> bool {
    is_square(8 * n + 1)
}

fn is_square(n: u64) -> bool {
    [0, 1, 4, 9].contains(&(n & 0xf)) && {
        let s: u64 = unsafe { (n as f64).sqrt().to_int_unchecked() };
        s * s == n
    }
}
