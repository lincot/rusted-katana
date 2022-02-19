#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const ARR: [i64; 20] = [
    18, 36, 20, 29, 72, 69, 10, 82, 16, 30, 40, 54, 22, 68, 89, 60, 49, 67, 96, 28,
];

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&ARR);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::add(arr));
        }
    })
}
