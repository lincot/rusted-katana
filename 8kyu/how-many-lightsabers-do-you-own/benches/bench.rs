#![feature(test)]

extern crate test;
use how_many_lightsabers_do_you_own::how_many_lightsabers_do_you_own;
use test::{black_box, Bencher};

#[bench]
fn bench_zach(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(how_many_lightsabers_do_you_own(black_box("Zach")));
        }
    });
}

#[bench]
fn bench_john(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(how_many_lightsabers_do_you_own(black_box("John")));
        }
    });
}
