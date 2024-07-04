#![feature(test)]

extern crate test;
use core::array;
use gravity_flip_3d_version::flip;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let cubes: [_; if cfg!(miri) { 4 } else { 32 }] =
        array::from_fn(|_| array::from_fn::<_, 32, _>(|_| rng.gen_range(0..100)).into());
    bencher.iter(|| flip(black_box('U'), black_box(cubes.to_vec())));
}
