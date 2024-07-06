#![feature(test)]

extern crate test;
use is_it_a_letter::is_it_letter;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(is_it_letter(black_box('a')));
        }
    });
}
