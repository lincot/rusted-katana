#![no_std]
#![feature(test)]

extern crate test;
use correct_the_time_string::time_correct;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        time_correct(black_box("20:40:39"));
        time_correct(black_box("09:10:01"));
        time_correct(black_box("11:70:10"));
        time_correct(black_box("19:99:09"));
        time_correct(black_box("19:99:99"));
        time_correct(black_box("24:01:01"));
        time_correct(black_box("52:01:01"));
        time_correct(black_box("00:1c:22"));
    });
}
