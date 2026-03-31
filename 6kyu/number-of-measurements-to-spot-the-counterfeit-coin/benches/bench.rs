#![feature(test)]

extern crate test;
use core::array;

use number_of_measurements_to_spot_the_counterfeit_coin::how_many_measurements;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let ns: [_; 100] = array::from_fn(|_| rng.random());
    bencher.iter(|| {
        for n in ns {
            black_box(how_many_measurements(black_box(n)));
        }
    });
}
