#![feature(test)]

extern crate test;
use closest_pair_of_points_in_linearithmic_time::closest_pair;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let points: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|_| {
        (
            rng.random_range(-100f64..100.),
            rng.random_range(-100f64..100.),
        )
    });
    bencher.iter(|| closest_pair(black_box(&points)));
}
