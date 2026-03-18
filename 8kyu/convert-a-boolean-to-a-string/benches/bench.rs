#![feature(test)]

extern crate test;
use convert_a_boolean_to_a_string::boolean_to_string;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| boolean_to_string(black_box(true)));
}
