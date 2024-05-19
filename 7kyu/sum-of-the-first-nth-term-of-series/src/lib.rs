//! <https://www.codewars.com/kata/555eded1ad94b00403000071/train/rust>

pub fn series_sum(n: u32) -> String {
    let sum: f64 = (0..n).map(|x| 1. / (1 + 3 * x) as f64).sum();
    let sum = sum.mul_add(100., 0.5) as usize;
    let res = vec![
        b'0' + (sum / 100) as u8,
        b'.',
        b'0' + (sum / 10 % 10) as u8,
        b'0' + (sum % 10) as u8,
    ];
    unsafe { String::from_utf8_unchecked(res) }
}
