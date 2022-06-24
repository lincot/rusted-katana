#![feature(test)]

extern crate test;
use convert_a_string_to_an_array::string_to_array;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("I love arrays they are my favorite");
    bencher.iter(|| string_to_array(s));
}
