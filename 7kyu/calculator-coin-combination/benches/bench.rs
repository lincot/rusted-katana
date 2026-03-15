#![feature(test)]

extern crate test;
use calculator_coin_combination::coin_combo;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| coin_combo(black_box(5000)));
}
