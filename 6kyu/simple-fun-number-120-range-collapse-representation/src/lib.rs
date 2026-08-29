//! <https://www.codewars.com/kata/589d6bc33b368ea992000035/train/rust>

pub fn descriptions(arr: &[u32]) -> u32 {
    arr.array_windows()
        .map(|&[a, b]| if a + 1 == b { 2 } else { 1 })
        .product()
}
