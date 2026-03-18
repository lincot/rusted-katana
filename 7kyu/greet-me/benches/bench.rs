#![feature(test)]

extern crate test;
use greet_me::greet;
use test::{Bencher, black_box};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        for name in ["riley", "JACK", "aliCE"] {
            greet(black_box(name));
        }
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        for name in ["райли", "ДЖЕК", "алиСА"] {
            greet(black_box(name));
        }
    });
}
