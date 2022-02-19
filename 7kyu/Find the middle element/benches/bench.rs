#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input_array = black_box([1, 2, 3]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::gimme(input_array));
        }
    })
}
