#![feature(test)]

extern crate test;
use spinning_rings_fidget_spinner_edition::spinning_rings;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for inner_max in 1..100 {
            for outer_max in 1..100 {
                black_box(spinning_rings(black_box(inner_max), black_box(outer_max)));
            }
        }
    });
}
