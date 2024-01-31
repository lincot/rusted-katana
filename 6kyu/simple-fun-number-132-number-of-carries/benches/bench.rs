#![feature(test)]

extern crate test;
use simple_fun_number_132_number_of_carries::number_of_carries;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for a in 0..if cfg!(miri) { 10 } else { 1000 } {
            for b in 0..if cfg!(miri) { 10 } else { 1000 } {
                black_box(number_of_carries(black_box(a), black_box(b)));
            }
        }
    });
}
