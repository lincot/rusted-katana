#![feature(test)]

extern crate test;
use std::collections::HashMap;

use my_language_skills::my_languages;
use test::{Bencher, black_box};

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
    bencher.iter(|| {
        for _ in 0..3 {
            black_box(my_languages(black_box(results.clone())));
        }
    });
}
