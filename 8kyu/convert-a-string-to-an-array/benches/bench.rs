#![feature(test)]

extern crate test;
use convert_a_string_to_an_array::string_to_array;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| string_to_array(black_box("I love arrays they are my favorite")));
}
