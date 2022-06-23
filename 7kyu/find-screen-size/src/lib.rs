//! <https://www.codewars.com/kata/5bbd279c8f8bbd5ee500000f/train/rust>

pub fn find_screen_height(width: u64, ratio: &str) -> String {
    let (vertical, horizontal) = ratio.split_once(':').unwrap();

    let vertical = vertical.parse::<u64>().unwrap();
    let horizontal = horizontal.parse::<u64>().unwrap();

    let height = width * horizontal / vertical;

    format!("{}x{}", width, height)
}
