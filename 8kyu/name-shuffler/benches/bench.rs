#![feature(test)]

extern crate test;
use name_shuffler::name_shuffler;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("Дмитрий Муратов");
    bencher.iter(|| name_shuffler(s));
}
