#![feature(test)]

extern crate test;
use coprimes_up_to_n::coprimes;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| coprimes(black_box(if cfg!(miri) { 50 } else { 1002 })));
}
