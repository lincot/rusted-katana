//! <https://www.codewars.com/kata/5601409514fc93442500010b/train/rust>

pub fn better_than_average(class_points: &[u16], your_points: u16) -> bool {
    class_points.iter().sum::<u16>() < your_points * class_points.len() as u16
}
