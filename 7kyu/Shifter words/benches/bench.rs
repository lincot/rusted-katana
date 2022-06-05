#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("S CAGE GLAD O UB ZC ION MOON I OX P CLAD OX BIG FKL YE OPY CAKE LYME");
    bencher.iter(|| solution::shifter(s));
}
