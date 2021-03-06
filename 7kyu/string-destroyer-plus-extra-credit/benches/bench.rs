#![feature(test)]

extern crate test;
use string_destroyer_plus_extra_credit::destroy;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input_sets = black_box(vec![
        ['B', 'b'].iter().copied().collect(),
        ['C', 'm', 'f'].iter().copied().collect(),
    ]);
    bencher.iter(|| destroy(input_sets.clone()));
}
