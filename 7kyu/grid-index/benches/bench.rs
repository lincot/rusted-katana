#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use grid_index::grid_index;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let grid = [
        vec!['h', 'e', 'l', 'l'],
        vec!['o', 'c', 'o', 'd'],
        vec!['e', 'w', 'a', 'r'],
        vec!['r', 'i', 'o', 'r'],
    ];
    let grid = black_box(&grid);
    let indices = black_box(&[5, 7, 9, 3, 6, 6, 8, 8, 16, 13]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(grid_index(grid, indices));
        }
    });
}
