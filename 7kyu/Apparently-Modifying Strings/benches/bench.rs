#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let string = black_box("It was great and I have never been on live television before but sometimes I dont watch this.");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::apparently(string));
        }
    })
}
