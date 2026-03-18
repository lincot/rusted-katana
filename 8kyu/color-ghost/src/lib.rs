//! <https://www.codewars.com/kata/53f1015fa9fe02cbda00111a/train/rust>

use rand::{RngExt, rng};

pub struct Ghost {
    pub color: &'static str,
}

impl Ghost {
    pub fn new() -> Self {
        let color = match rng().random::<u8>() % 4 {
            0 => "white",
            1 => "yellow",
            2 => "purple",
            3 => "red",
            _ => unreachable!(),
        };
        Self { color }
    }
}
