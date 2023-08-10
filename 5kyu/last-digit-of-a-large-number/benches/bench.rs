#![no_std]
#![feature(test)]

extern crate test;
use last_digit_of_a_large_number::last_digit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1372"), black_box("97376")));
            black_box(last_digit(black_box("1373"), black_box("97376")));
            black_box(last_digit(black_box("1374"), black_box("97376")));
            black_box(last_digit(black_box("1375"), black_box("97376")));
            black_box(last_digit(black_box("1376"), black_box("97376")));
            black_box(last_digit(black_box("1377"), black_box("97376")));
            black_box(last_digit(black_box("1378"), black_box("97376")));
            black_box(last_digit(black_box("1379"), black_box("97376")));
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1372"), black_box("97376")));
            black_box(last_digit(black_box("1373"), black_box("97376")));
            black_box(last_digit(black_box("1374"), black_box("97376")));
            black_box(last_digit(black_box("1375"), black_box("97376")));
            black_box(last_digit(black_box("1376"), black_box("97376")));
            black_box(last_digit(black_box("1377"), black_box("97376")));
            black_box(last_digit(black_box("1378"), black_box("97376")));
            black_box(last_digit(black_box("1379"), black_box("97376")));
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1372"), black_box("97376")));
            black_box(last_digit(black_box("1373"), black_box("97376")));
            black_box(last_digit(black_box("1374"), black_box("97376")));
            black_box(last_digit(black_box("1375"), black_box("97376")));
            black_box(last_digit(black_box("1376"), black_box("97376")));
            black_box(last_digit(black_box("1377"), black_box("97376")));
            black_box(last_digit(black_box("1378"), black_box("97376")));
            black_box(last_digit(black_box("1379"), black_box("97376")));
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1371"), black_box("97376")));
            black_box(last_digit(black_box("1372"), black_box("97376")));
            black_box(last_digit(black_box("1373"), black_box("97376")));
            black_box(last_digit(black_box("1374"), black_box("97376")));
            black_box(last_digit(black_box("1375"), black_box("97376")));
            black_box(last_digit(black_box("1376"), black_box("97376")));
            black_box(last_digit(black_box("1377"), black_box("97376")));
            black_box(last_digit(black_box("1378"), black_box("97376")));
            black_box(last_digit(black_box("1379"), black_box("97376")));
        }
    });
}
