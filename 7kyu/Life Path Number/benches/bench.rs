#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const EINSTEIN: &str = "1879-03-14";
const ELON_MUSK: &str = "1971-06-28";

#[bench]
fn bench_einstein(bencher: &mut Bencher) {
    let s = black_box(EINSTEIN);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::life_path_number(s));
        }
    })
}

#[bench]
fn bench_elon_musk(bencher: &mut Bencher) {
    let s = black_box(ELON_MUSK);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::life_path_number(s));
        }
    })
}
