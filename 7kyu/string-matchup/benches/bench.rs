#![feature(test)]

extern crate test;
use string_matchup::match_counts;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        match_counts(
            black_box(&[
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
            ]),
            black_box(&["abc".into(), "cde".into(), "uap".into()]),
        )
    });
}
