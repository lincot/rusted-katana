#![no_std]
#![feature(test)]

extern crate test;
use disemvowel_trolls::disemvowel;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| disemvowel(black_box("This website is for losers LOL!")));
}
