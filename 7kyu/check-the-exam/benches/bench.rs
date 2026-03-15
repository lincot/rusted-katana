#![feature(test)]

extern crate test;
use check_the_exam::check_exam;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        check_exam(
            black_box(&["a", "a", "b", "b", "b", "c", "b", "a"]),
            black_box(&["a", "c", "b", "d", "", "a", "a", "c"]),
        )
    });
}
