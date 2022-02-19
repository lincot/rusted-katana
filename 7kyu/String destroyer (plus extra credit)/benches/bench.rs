#![feature(test)]

extern crate test;
use std::collections::HashSet;
use test::{black_box, Bencher};

fn get_input_sets() -> Vec<HashSet<char>> {
    vec![
        ['B', 'b'].iter().cloned().collect(),
        ['C', 'm', 'f'].iter().cloned().collect(),
    ]
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let input_sets = black_box(get_input_sets());

    bencher.iter(|| solution::destroy(input_sets.clone()))
}
