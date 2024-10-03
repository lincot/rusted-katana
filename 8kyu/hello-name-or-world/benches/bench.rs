#![feature(test)]

extern crate test;
use hello_name_or_world::hello;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        for name in ["riley", "JACK", "aliCE"] {
            hello(black_box(name));
        }
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        for name in ["райли", "ДЖЕК", "алиСА"] {
            hello(black_box(name));
        }
    });
}
