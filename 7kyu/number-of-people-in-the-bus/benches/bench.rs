#![feature(test)]

extern crate test;
use core::array;
use number_of_people_in_the_bus::number;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let bus_stops: [_; 10] =
        array::from_fn(|_| (rng.random_range(0..=20), rng.random_range(0..=20)));
    bencher.iter(|| number(black_box(&bus_stops)));
}
