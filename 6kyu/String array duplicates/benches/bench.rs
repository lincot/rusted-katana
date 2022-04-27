#![feature(test)]

extern crate test;
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
    bencher.iter(|| solution::dup(arry.to_vec()))
}
