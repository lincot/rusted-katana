#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use word_values::word_value;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        word_value(black_box(&[
            "fktkssg",
            "hbiwazqjashd",
            "juxawyzbchqma",
            "iobcgaj",
            "yg",
            "xqjqpcqtehx",
            "usndhongymaou",
            "xabb",
            "c",
            "fxcns",
            "yqrbkejz",
            "lfrzubrxk",
        ]))
    });
}
