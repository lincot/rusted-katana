#![feature(test)]

extern crate test;
use lojban_numbers::convert_lojban;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(convert_lojban(black_box("renonorecivozenovocirepa")));
        }
    });
}
