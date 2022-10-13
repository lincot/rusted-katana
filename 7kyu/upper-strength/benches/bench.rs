#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use upper_strength::alex_mistakes;

#[bench]
fn bench(bencher: &mut Bencher) {
    let number_of_katas = black_box(9);
    let time_limit = black_box(180);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(alex_mistakes(number_of_katas, time_limit));
        }
    });
}
