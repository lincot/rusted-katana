//! <https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust>

pub fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&x| x).count() as u8
}
