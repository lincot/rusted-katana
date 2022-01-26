//! <https://www.codewars.com/kata/58649884a1659ed6cb000072/train/rust>

pub fn update_light(current: &str) -> String {
    match current.bytes().next().unwrap() {
        b'g' => "yellow",
        b'y' => "red",
        b'r' => "green",
        _ => panic!(),
    }
    .into()
}
