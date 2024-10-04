//! <https://www.codewars.com/kata/55e2adece53b4cdcb900006c/train/rust>

pub fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v2 <= v1 {
        return None;
    }

    let total_seconds = (g * 3600) / (v2 - v1);
    let mut minutes = total_seconds / 60;
    let hours = minutes / 60;
    minutes %= 60;
    let seconds = total_seconds % 60;

    Some(vec![hours, minutes, seconds])
}
