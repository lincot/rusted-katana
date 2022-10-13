//! <https://www.codewars.com/kata/5882b052bdeafec15e0000e6/train/rust>

#![no_std]

pub struct Quark<'a, 'b> {
    color: &'a str,
    flavor: &'b str,
}

impl<'a, 'b> Quark<'a, 'b> {
    pub const fn new(color: &'a str, flavor: &'b str) -> Self {
        Self { color, flavor }
    }

    pub const fn color(&self) -> &str {
        self.color
    }

    pub const fn flavor(&self) -> &str {
        self.flavor
    }

    pub const fn baryon_number(&self) -> f64 {
        0.333_333_333_333_333_3
    }

    pub fn interact(&mut self, other: &mut Self) {
        (self.color, other.color) = (other.color, self.color);
    }
}
