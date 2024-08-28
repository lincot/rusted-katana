#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use what_century_is_it::what_century;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(what_century(black_box("1999")));
        black_box(what_century(black_box("2011")));
        black_box(what_century(black_box("2154")));
        black_box(what_century(black_box("2259")));
        black_box(what_century(black_box("1234")));
        black_box(what_century(black_box("1023")));
        black_box(what_century(black_box("2000")));
        black_box(what_century(black_box("3210")));
    });
}
