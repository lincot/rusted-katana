#![feature(test)]

extern crate test;
use name_array_capping::cap_me;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        cap_me(black_box(vec![
            "riley".into(),
            "JACK".into(),
            "aliCE".into(),
        ]))
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        cap_me(black_box(vec![
            "райли".into(),
            "ДЖЕК".into(),
            "алиСА".into(),
            "Jarosław".into(),
        ]))
    });
}
