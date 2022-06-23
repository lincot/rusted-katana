#![feature(test)]

extern crate test;
use all_inclusive::contain_all_rots;
use test::{black_box, Bencher};

#[bench]
fn bench_same_length(bencher: &mut Bencher) {
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
    bencher.iter(|| contain_all_rots(strng, arr.to_vec()));
}

#[bench]
fn bench_different_length(bencher: &mut Bencher) {
    let strng = black_box("bsjq");
    let arr = black_box(&[
        "bsjq",
        "qbsj",
        "sjqb",
        "twZNsslC",
        "jqbs",
        "sjqbsjqbsjqbsjqbsjqbsjqb",
    ]);
    bencher.iter(|| contain_all_rots(strng, arr.to_vec()));
}
