#![feature(test)]

extern crate test;
use core::array;
use delta_generators::delta;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let values: [_; 1024] = array::from_fn(|_| rng.gen_range(-100i64..100));
    let level = 50;
    bencher.iter(|| delta(black_box(values), black_box(level)).collect::<Vec<_>>());
}
