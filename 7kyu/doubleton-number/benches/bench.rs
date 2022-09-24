#![no_std]
#![feature(test)]

extern crate test;
use doubleton_number::doubleton;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for i in 1..=1_000_000 {
            black_box(doubleton(i));
        }
    });
}
