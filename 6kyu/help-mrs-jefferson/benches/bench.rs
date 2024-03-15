#![feature(test)]

extern crate test;
use help_mrs_jefferson::shortest_arrang;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(shortest_arrang(black_box(10)));
        black_box(shortest_arrang(black_box(14)));
        black_box(shortest_arrang(black_box(16)));
        black_box(shortest_arrang(black_box(22)));
        black_box(shortest_arrang(black_box(65)));
        black_box(shortest_arrang(black_box(1874)));
        black_box(shortest_arrang(black_box(78_382_376)));
        black_box(shortest_arrang(black_box(28)));
    });
}
