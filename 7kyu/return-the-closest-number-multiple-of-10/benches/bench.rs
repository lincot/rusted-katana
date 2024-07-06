#![feature(test)]

extern crate test;
use return_the_closest_number_multiple_of_10::closest_multiple_of_10;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| closest_multiple_of_10(black_box(866_703)));
}
