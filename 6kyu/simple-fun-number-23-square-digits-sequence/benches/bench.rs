#![feature(test)]

extern crate test;
use simple_fun_number_23_square_digits_sequence::square_digits_sequence;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for a0 in 1..=if cfg!(miri) { 20 } else { 650 } {
            black_box(square_digits_sequence(black_box(a0)));
        }
    });
}
