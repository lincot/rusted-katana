//! <https://www.codewars.com/kata/59f6a855bee845d3cd000046/train/rust>

#![no_std]

mod preloaded;

extern crate alloc;
use alloc::{string::String, vec::Vec};
use preloaded::{Coffee, Milk, Sugar};

pub struct CoffeeBuilder(Coffee);

impl CoffeeBuilder {
    pub fn new() -> Self {
        Self(Coffee {
            sort: String::with_capacity("Americano".len()),
            milk: Vec::with_capacity(1),
            sugar: Vec::with_capacity(1),
        })
    }

    pub fn set_black_coffee(mut self) -> Self {
        self.0.sort = "Black".into();
        self
    }

    pub fn set_cubano_coffee(mut self) -> Self {
        self.0.sort = "Cubano".into();
        self.0.sugar.push(Sugar {
            sort: "Brown".into(),
        });
        self
    }

    pub fn set_antoccino_coffee(mut self) -> Self {
        self.0.sort = "Americano".into();
        self.0.milk.push(Milk { fat: 0.5 });
        self
    }

    pub fn with_milk(mut self, fat: f32) -> Self {
        self.0.milk.push(Milk { fat });
        self
    }

    pub fn with_sugar(mut self, sort: String) -> Self {
        self.0.sugar.push(Sugar { sort });
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Coffee {
        self.0
    }
}
