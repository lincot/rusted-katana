#![no_std]
#![feature(test)]

extern crate test;
use simple_fun_number_23_square_digits_sequence::square_digits_sequence;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let to = black_box(650);
    bencher.iter(|| {
        for a0 in 1..=to {
            black_box(square_digits_sequence(a0));
        }
    });
}
