#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "S CAGE GLAD O UB ZC ION MOON I OX P CLAD OX BIG FKL YE OPY CAKE LYME";

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(S);

    bencher.iter(|| solution::shifter(s))
}
