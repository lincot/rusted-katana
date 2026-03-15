#![feature(test)]

extern crate test;
use refactored_greeting::Person;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| Person::new(black_box("Alice")).greet(black_box("Bob")));
}
