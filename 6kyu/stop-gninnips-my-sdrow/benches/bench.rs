#![feature(test)]

extern crate test;
use stop_gninnips_my_sdrow::spin_words;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        spin_words(black_box(
            "Число сочетаний из n по k равно биномиальному коэффициенту",
        ))
    });
}
