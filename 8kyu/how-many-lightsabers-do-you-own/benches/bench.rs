#![feature(test)]

extern crate test;
use how_many_lightsabers_do_you_own::how_many_lightsabers_do_you_own;
use test::{black_box, Bencher};

#[bench]
fn bench_zach(bencher: &mut Bencher) {
    let name = black_box("Zach");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(how_many_lightsabers_do_you_own(name));
        }
    });
}

#[bench]
fn bench_john(bencher: &mut Bencher) {
    let name = black_box("John");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(how_many_lightsabers_do_you_own(name));
        }
    });
}
