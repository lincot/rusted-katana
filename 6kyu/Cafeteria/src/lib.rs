//! <https://www.codewars.com/kata/59f6a855bee845d3cd000046/train/rust>

mod preloaded;
use preloaded::{Coffee, Milk, Sugar};

pub struct CoffeeBuilder {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

impl CoffeeBuilder {
    pub const fn new() -> Self {
        Self {
            sort: String::new(),
            milk: vec![],
            sugar: vec![],
        }
    }

    pub fn set_black_coffee(mut self) -> Self {
        self.sort = "Black".into();
        self
    }

    pub fn set_cubano_coffee(mut self) -> Self {
        self.sort = "Cubano".into();
        self.sugar.push(Sugar {
            sort: "Brown".into(),
        });
        self
    }

    pub fn set_antoccino_coffee(mut self) -> Self {
        self.sort = "Americano".into();
        self.milk.push(Milk { fat: 0.5 });
        self
    }

    pub fn with_milk(mut self, fat: f32) -> Self {
        self.milk.push(Milk { fat });
        self
    }

    pub fn with_sugar(mut self, sort: String) -> Self {
        self.sugar.push(Sugar { sort });
        self
    }

    pub fn build(self) -> Coffee {
        Coffee {
            sort: self.sort,
            milk: self.milk,
            sugar: self.sugar,
        }
    }
}
