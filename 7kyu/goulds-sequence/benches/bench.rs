#![feature(test)]

extern crate test;
use goulds_sequence::gould;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| gould().take(black_box(16)).collect::<Box<[_]>>());
}
