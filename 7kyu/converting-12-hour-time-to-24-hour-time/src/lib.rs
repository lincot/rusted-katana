//! <https://www.codewars.com/kata/59b0a6da44a4b7080300008a/train/rust>

pub fn to24hourtime(hour: u8, minute: u8, period: &str) -> String {
    let hour = if hour == 12 { 0 } else { hour } + if period.starts_with('p') { 12 } else { 0 };
    unsafe {
        String::from_utf8_unchecked(vec![
            b'0' + hour / 10,
            b'0' + hour % 10,
            b'0' + minute / 10,
            b'0' + minute % 10,
        ])
    }
}
