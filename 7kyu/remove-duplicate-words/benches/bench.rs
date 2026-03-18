#![feature(test)]

extern crate test;
use remove_duplicate_words::remove_duplicate_words;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const S: &str =
        "альфа бета бета гамма гамма гамма дельта гамма гамма дельта объект причина причина мнение";

    bencher.iter(|| remove_duplicate_words(black_box(S)));
}
