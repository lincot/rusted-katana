#![feature(test)]

extern crate test;
use most_frequent_weekdays::most_frequent_days;
use test::{black_box, Bencher};

#[bench]
fn bench_1984(bencher: &mut Bencher) {
    let year = black_box(1984);
    bencher.iter(|| most_frequent_days(year));
}

#[bench]
fn bench_2001(bencher: &mut Bencher) {
    let year = black_box(2001);
    bencher.iter(|| most_frequent_days(year));
}

#[bench]
fn bench_2016(bencher: &mut Bencher) {
    let year = black_box(2016);
    bencher.iter(|| most_frequent_days(year));
}
