#![feature(test)]

extern crate test;
use string_array_duplicates::dup;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arry = [
        "adanac".into(),
        "soonness".into(),
        "toolless".into(),
        "ppellee".into(),
    ];
    let arry = black_box(&arry);
    bencher.iter(|| dup(arry.to_vec()));
}
