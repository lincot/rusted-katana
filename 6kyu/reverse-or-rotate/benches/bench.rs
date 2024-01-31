#![feature(test)]

extern crate test;
use reverse_or_rotate::revrot;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(revrot(
                black_box("1994033775182780067155464327690480265895"),
                black_box(5),
            ));
        }
    });
}
