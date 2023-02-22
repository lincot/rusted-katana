#![no_std]
#![feature(test)]

extern crate test;
use number_of_measurements_to_spot_the_counterfeit_coin::how_many_measurements;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for n in 1..1_000_000 {
            black_box(how_many_measurements(black_box(n)));
        }
    });
}
