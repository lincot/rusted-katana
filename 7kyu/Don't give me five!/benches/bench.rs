#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const START: isize = 456439;
const END: isize = 1837815;

#[bench]
fn bench(bencher: &mut Bencher) {
    let start = black_box(START);
    let end = black_box(END);

    bencher.iter(|| solution::dont_give_me_five(start, end))
}
