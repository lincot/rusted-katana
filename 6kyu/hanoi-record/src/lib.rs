//! <https://www.codewars.com/kata/534eb5ad704a49dcfa000ba6/train/rust>

pub const fn hanoi(disks: u8) -> u128 {
    u128::MAX >> (u128::BITS - disks as u32)
}
