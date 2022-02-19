#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

fn get_a1() -> [String; 25] {
    [
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
    ]
}

fn get_a2() -> [String; 3] {
    ["abc".into(), "cde".into(), "uap".into()]
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let a1 = get_a1();
    let a1 = black_box(&a1);
    let a2 = get_a2();
    let a2 = black_box(&a2);

    bencher.iter(|| solution::match_counts(a1, a2))
}
