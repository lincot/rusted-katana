#![feature(test)]

extern crate test;
use core::array;
use multiples_by_permutations_ii::find_lowest_int;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let ks: [_; if cfg!(miri) { 1 } else { 100 }] =
        array::from_fn(|_| rng.gen_range(10..10u64.pow(15)));
    bencher.iter(|| {
        for k in ks {
            black_box(find_lowest_int(black_box(k)));
        }
    });
}
