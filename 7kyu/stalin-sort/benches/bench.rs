#![feature(test)]

extern crate test;
use stalin_sort::stalin_sort;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let vector: Vec<_> = (0..if cfg!(miri) { 16 } else { 1024 })
        .map(|i| if i % 2 == 0 { i * 5 } else { i })
        .collect();
    bencher.iter(|| stalin_sort(black_box(&mut vector.clone())));
}
