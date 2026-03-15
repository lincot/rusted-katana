#![feature(test)]

extern crate test;
use chain_me::chain;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let functions: [Box<dyn Fn(_) -> _>; 2] = [Box::new(|x| x * 30), Box::new(|x| x + 10)];
    bencher.iter(|| chain(black_box(50), black_box(&functions)));
}
