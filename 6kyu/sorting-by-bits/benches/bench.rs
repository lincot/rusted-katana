#![feature(test)]

extern crate test;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sorting_by_bits::sort_by_bit;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    // most upvoted solution needs a Vec
    let arr: Vec<_> = (0..if cfg!(miri) { 50 } else { 1000 })
        .map(|_| rng.random())
        .collect();
    bencher.iter(|| {
        let mut arr = arr.clone();
        sort_by_bit(black_box(&mut arr));
        arr
    });
}
