//! <https://www.codewars.com/kata/5121303128ef4b495f000001/train/rust>

pub struct Person<'a> {
    pub name: &'a str,
}

impl<'a> Person<'a> {
    pub const fn new(name: &'a str) -> Self {
        Person { name }
    }

    pub fn greet(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}
