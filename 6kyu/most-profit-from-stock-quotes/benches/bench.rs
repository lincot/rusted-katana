#![feature(test)]

extern crate test;
use core::array;
use most_profit_from_stock_quotes::max_profit;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let quotes: [_; 100] = array::from_fn(|_| rng.gen_range(0..10_000));
    bencher.iter(|| max_profit(black_box(&quotes)));
}
