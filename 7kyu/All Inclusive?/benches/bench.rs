#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let strng = black_box("cyBmigfJMk6z");
    let arr = black_box(&[
        "Mk6zcyBmigfJ",
        "fJMk6zcyBmig",
        "JMk6zcyBmigf",
        "6zcyBmigfJMk",
        "k6zcyBmigfJM",
        "cyBmigfJMk6z",
        "gfJMk6zcyBmi",
        "igfJMk6zcyBm",
        "migfJMk6zcyB",
        "zcyBmigfJMk6",
        "BmigfJMk6zcy",
        "yBmigfJMk6zc",
    ]);
    bencher.iter(|| solution::contain_all_rots(strng, arr.to_vec()));
}
