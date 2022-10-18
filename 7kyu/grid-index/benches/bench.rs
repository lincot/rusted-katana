#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use grid_index::grid_index;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(grid_index(
                black_box(&[
                    vec!['h', 'e', 'l', 'l'],
                    vec!['o', 'c', 'o', 'd'],
                    vec!['e', 'w', 'a', 'r'],
                    vec!['r', 'i', 'o', 'r'],
                ]),
                black_box(&[5, 7, 9, 3, 6, 6, 8, 8, 16, 13]),
            ));
        }
    });
}
