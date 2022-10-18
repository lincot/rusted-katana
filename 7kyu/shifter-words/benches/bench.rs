#![no_std]
#![feature(test)]

extern crate test;
use shifter_words::shifter;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        shifter(black_box(
            "S CAGE GLAD O UB ZC ION MOON I OX P CLAD OX BIG FKL YE OPY CAKE LYME",
        ))
    });
}
