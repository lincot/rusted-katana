#![feature(test)]

extern crate test;
use tank_truck::tank_vol;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| tank_vol(black_box(5), black_box(7), black_box(3848)));
}
