#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const ARR: [&str; 5] = ["I", "wish", "I", "hadn't", "come"];

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(ARR);

    bencher.iter(|| solution::part_list(arr.to_vec()))
}
