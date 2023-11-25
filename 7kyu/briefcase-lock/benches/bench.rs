#![no_std]
#![feature(test)]

extern crate test;
use briefcase_lock::min_turns;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(min_turns(black_box("7109"), black_box("2332")));
            black_box(min_turns(black_box("2391"), black_box("4984")));
            black_box(min_turns(black_box("4089"), black_box("5672")));
        }
    });
}
