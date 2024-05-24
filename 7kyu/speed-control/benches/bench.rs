#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use speed_control::gps;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let x: [_; 50] = array::from_fn(|_| rng.gen_range(0f64..16.));
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(gps(black_box(12), black_box(x.into())));
        }
    });
}
