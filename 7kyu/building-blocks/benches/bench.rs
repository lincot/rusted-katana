#![feature(test)]

extern crate test;
use building_blocks::Block;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        let block = Block::new(black_box(&[2, 4, 6]));
        black_box(black_box(&block).get_width());
        black_box(black_box(&block).get_length());
        black_box(black_box(&block).get_height());
        black_box(black_box(&block).get_volume());
        black_box(black_box(&block).get_surface_area());
    });
}
