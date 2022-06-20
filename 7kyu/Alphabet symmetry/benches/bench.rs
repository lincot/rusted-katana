#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let strings = [
        "IAMDEFANDJKL".into(),
        "thedefgh".into(),
        "xyzDEFghijabc".into(),
    ];
    let strings = black_box(&strings);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::solve(strings));
        }
    });
}
