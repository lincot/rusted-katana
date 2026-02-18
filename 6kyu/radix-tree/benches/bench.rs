#![feature(test)]

extern crate test;
use radix_tree::radix_tree;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        radix_tree(black_box(&[
            "romane",
            "romanus",
            "romulus",
            "rubens",
            "rubicon",
            "rubicundus",
        ]))
    });
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| {
        radix_tree(black_box(&[
            "романе",
            "романус",
            "ромулус",
            "рубенс",
            "рубикон",
            "рубикундус",
        ]))
    });
}
