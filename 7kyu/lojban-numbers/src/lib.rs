//! <https://www.codewars.com/kata/6584b7cac29ca91dd9124009/train/rust>

pub fn convert_lojban(input: &str) -> u64 {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|chunk| match chunk[0] {
            b'n' => 0,
            b'p' => 1,
            b'r' => 2,
            b'c' => 3,
            b'v' => 4,
            b'm' => 5,
            b'x' => 6,
            b'z' => 7,
            b'b' => 8,
            b's' => 9,
            _ => panic!(),
        })
        .fold(0, |acc, d| 10 * acc + d)
}
