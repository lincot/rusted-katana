#![feature(test)]

extern crate test;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sort_numbers::sort_numbers;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    // most upvoted needs a vec
    let arr: Vec<_> = (0..300).map(|_| rng.random()).collect();
    bencher.iter(|| sort_numbers(black_box(&arr)));
}
