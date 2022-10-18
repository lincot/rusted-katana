#![no_std]
#![feature(test)]

extern crate test;
use even_odd_disparity::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        solve(black_box(&[
            "5".into(),
            "15".into(),
            "16".into(),
            "10".into(),
            "6".into(),
            "4".into(),
            "16".into(),
            "t".into(),
            "13".into(),
            "n".into(),
            "14".into(),
            "k".into(),
            "n".into(),
            "0".into(),
            "q".into(),
            "d".into(),
            "7".into(),
            "9".into(),
        ]))
    });
}
