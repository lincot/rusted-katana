#![feature(test)]

extern crate test;
use positions_average::pos_average;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        pos_average(black_box(
            "466960, 069060, 494940, 060069, 060090, 640009, 496464, 606900, 004000, 944096",
        ));
    });
}
