//! <https://www.codewars.com/kata/5f25f475420f1b002412bb1f/train/rust>

pub fn find_the_number_plate(n: u32) -> String {
    let number = (n % 999) + 1;
    let base = n / 999;
    let res = vec![
        b'a' + ((base % 26) as u8),
        b'a' + ((base / 26 % 26) as u8),
        b'a' + ((base / (26 * 26) % 26) as u8),
        b'0' + (number / 100) as u8,
        b'0' + (number % 100 / 10) as u8,
        b'0' + (number % 10) as u8,
    ];
    unsafe { String::from_utf8_unchecked(res) }
}
