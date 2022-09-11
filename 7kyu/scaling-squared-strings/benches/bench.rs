#![feature(test)]

extern crate test;
use scaling_squared_strings::scale;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("abcd\nefgh\nijkl\nmnop\nqrst");
    let k = black_box(10);
    let n = black_box(10);
    bencher.iter(|| scale(s, k, n));
}
