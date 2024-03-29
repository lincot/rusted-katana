#![feature(test)]

extern crate test;
use doubleton_number::doubleton;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for num in 1..=if cfg!(miri) { 100 } else { 1_000_000 } {
            black_box(doubleton(black_box(num)));
        }
    });
}
