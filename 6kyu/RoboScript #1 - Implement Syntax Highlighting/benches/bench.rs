#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let code = black_box("FFFR345F2LL");
    bencher.iter(|| solution::highlight(code));
}
