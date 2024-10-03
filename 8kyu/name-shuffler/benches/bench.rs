#![feature(test)]

extern crate test;
use name_shuffler::name_shuffler;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| name_shuffler(black_box("Dmitry Muratov")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| name_shuffler(black_box("Дмитрий Муратов")));
}
