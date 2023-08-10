#![no_std]
#![feature(test)]

extern crate test;
use experimenting_with_a_sequence_of_complex_numbers::f;
use num_complex::Complex;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(f(black_box(Complex::new(0.61, 0.73)), black_box(1e-8)));
        }
    });
}
