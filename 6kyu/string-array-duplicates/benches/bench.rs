#![feature(test)]

extern crate test;
use string_array_duplicates::dup;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        dup(black_box(vec![
            "adanac".into(),
            "soonness".into(),
            "toolless".into(),
            "ppellee".into(),
        ]))
    });
}
