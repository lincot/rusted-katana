#![feature(test)]

extern crate test;
use count_the_digit::nb_dig;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| nb_dig(black_box(if cfg!(miri) { 50 } else { 5000 }), black_box(5)));
}
