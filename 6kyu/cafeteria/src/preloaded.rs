extern crate alloc;
use alloc::{string::String, vec::Vec};

#[derive(Debug)]
pub struct Coffee {
    pub sort: String,
    pub milk: Vec<Milk>,
    pub sugar: Vec<Sugar>,
}

#[derive(Debug)]
pub struct Milk {
    pub fat: f32,
}

#[derive(Debug)]
pub struct Sugar {
    pub sort: String,
}
