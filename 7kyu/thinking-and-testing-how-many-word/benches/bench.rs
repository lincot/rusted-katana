#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use thinking_and_testing_how_many_word::testit;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        testit(black_box(
            "When you in order to do something by a wrong way, your heart will missed somewhere.",
        ))
    });
}
