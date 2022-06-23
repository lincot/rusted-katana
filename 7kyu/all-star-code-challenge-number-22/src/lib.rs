//! <https://www.codewars.com/kata/5865cff66b5699883f0001aa/train/rust>

pub fn to_time(seconds: u32) -> String {
    let mut minutes = seconds / 60;
    let hours = minutes / 60;
    minutes %= 60;

    format!("{} hour(s) and {} minute(s)", hours, minutes)
}
