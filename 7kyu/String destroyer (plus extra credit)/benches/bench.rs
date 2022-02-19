#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input_sets = black_box(vec![
        ['B', 'b'].iter().cloned().collect(),
        ['C', 'm', 'f'].iter().cloned().collect(),
    ]);
    bencher.iter(|| solution::destroy(input_sets.clone()))
}
