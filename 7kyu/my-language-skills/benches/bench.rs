#![feature(test)]

extern crate test;
use my_language_skills::my_languages;
use std::collections::HashMap;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let results = HashMap::from([
        ("Swedish", 32),
        ("Estonian", 87),
        ("Irish", 21),
        ("Slovenian", 34),
        ("English", 48),
        ("Lithuanian", 73),
        ("Maltese", 100),
    ]);
    bencher.iter(|| my_languages(black_box(results.clone())));
}
