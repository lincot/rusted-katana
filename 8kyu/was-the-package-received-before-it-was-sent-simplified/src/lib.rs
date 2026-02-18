//! <https://www.codewars.com/kata/6707688c0f597511f6649270/train/rust>

pub const fn was_package_received_yesterday(
    tz_from: i32,
    tz_to: i32,
    start: i32,
    duration: i32,
) -> bool {
    start + duration + tz_to < tz_from
}
