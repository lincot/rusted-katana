#![feature(test)]

extern crate test;
use roboscript_number_1_implement_syntax_highlighting::highlight;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let code = black_box("FFFR345F2LL");
    bencher.iter(|| highlight(code));
}
