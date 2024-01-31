//! <https://www.codewars.com/kata/55b75fcf67e558d3750000a3/train/rust>

pub struct Block {
    width: u32,
    length: u32,
    height: u32,
}

impl Block {
    pub const fn new(&[width, length, height]: &[u32; 3]) -> Self {
        Self {
            width,
            length,
            height,
        }
    }

    pub const fn get_width(&self) -> u32 {
        self.width
    }

    pub const fn get_length(&self) -> u32 {
        self.length
    }

    pub const fn get_height(&self) -> u32 {
        self.height
    }

    pub const fn get_volume(&self) -> u32 {
        self.width * self.length * self.height
    }

    pub const fn get_surface_area(&self) -> u32 {
        2 * (self.width * self.length + self.length * self.height + self.height * self.width)
    }
}
