#![feature(test)]

extern crate test;
use get_planet_name_by_id::get_planet_name;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_planet_name(black_box(5)));
}
