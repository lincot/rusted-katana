#![no_std]
#![feature(test)]

extern crate test;
use life_path_number::life_path_number;
use test::{black_box, Bencher};

#[bench]
fn bench_einstein(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(life_path_number(black_box("1879-03-14")));
        }
    });
}

#[bench]
fn bench_elon_musk(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(life_path_number(black_box("1971-06-28")));
        }
    });
}
