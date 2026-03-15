#![feature(test)]

extern crate test;
use spinning_rings::spinning_rings;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for inner_max in 0..101 {
            for outer_max in 0..101 {
                spinning_rings(black_box(inner_max), black_box(outer_max));
            }
        }
    });
}
