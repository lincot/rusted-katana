#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a1 = [
        "abc".into(),
        "abc".into(),
        "xyz".into(),
        "abcd".into(),
        "cde".into(),
        "abc".into(),
        "abc".into(),
        "xyz".into(),
        "abcd".into(),
        "cde".into(),
        "abc".into(),
        "abc".into(),
        "xyz".into(),
        "abcd".into(),
        "cde".into(),
        "abc".into(),
        "abc".into(),
        "xyz".into(),
        "abcd".into(),
        "cde".into(),
        "abc".into(),
        "abc".into(),
        "xyz".into(),
        "abcd".into(),
        "cde".into(),
    ];
    let a1 = black_box(&a1);
    let a2 = ["abc".into(), "cde".into(), "uap".into()];
    let a2 = black_box(&a2);
    bencher.iter(|| solution::match_counts(a1, a2));
}
