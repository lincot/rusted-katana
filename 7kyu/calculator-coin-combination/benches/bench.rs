#![feature(test)]

extern crate test;
use calculator_coin_combination::coin_combo;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| coin_combo(black_box(5000)));
}
