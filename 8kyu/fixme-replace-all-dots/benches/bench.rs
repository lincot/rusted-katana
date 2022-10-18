#![no_std]
#![feature(test)]

extern crate test;
use fixme_replace_all_dots::replace_dots;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        replace_dots(black_box(
            "one.two.three.one.two.three.one.two.three.one.two.three.one.two.three",
        ))
    });
}
