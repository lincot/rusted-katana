#![feature(test)]

extern crate test;
use holiday_viii_duty_free::duty_free;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(duty_free(black_box(12), black_box(50), black_box(1000)));
        }
    });
}
