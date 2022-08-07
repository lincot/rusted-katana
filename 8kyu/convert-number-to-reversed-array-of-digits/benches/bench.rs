#![feature(test)]

extern crate test;
use convert_number_to_reversed_array_of_digits::digitize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(456_486_734_798);
    bencher.iter(|| digitize(n));
}
