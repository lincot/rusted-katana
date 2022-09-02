#![feature(test)]

extern crate test;
use disemvowel_trolls::disemvowel;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("This website is for losers LOL!");
    bencher.iter(|| disemvowel(s));
}
