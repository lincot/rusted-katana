#![feature(test)]

extern crate test;
use create_phone_number::create_phone_number;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| create_phone_number(black_box(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0])));
}
