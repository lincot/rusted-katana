#![feature(test)]

extern crate test;
use parse_nice_int_from_char_problem::get_age;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_age(black_box("7 years old")));
}
