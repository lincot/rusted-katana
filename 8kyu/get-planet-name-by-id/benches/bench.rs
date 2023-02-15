#![no_std]
#![feature(test)]

extern crate test;
use get_planet_name_by_id::get_planet_name;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_planet_name(black_box(5)));
        }
    });
}
