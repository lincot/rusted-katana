#![feature(test)]

extern crate test;
use most_frequent_weekdays::most_frequent_days;
use test::{Bencher, black_box};

#[bench]
fn bench_1984(bencher: &mut Bencher) {
    bencher.iter(|| most_frequent_days(black_box(1984)));
}

#[bench]
fn bench_2001(bencher: &mut Bencher) {
    bencher.iter(|| most_frequent_days(black_box(2001)));
}

#[bench]
fn bench_2016(bencher: &mut Bencher) {
    bencher.iter(|| most_frequent_days(black_box(2016)));
}
