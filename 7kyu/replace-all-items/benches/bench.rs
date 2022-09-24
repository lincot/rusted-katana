#![no_std]
#![feature(test)]

extern crate test;
use replace_all_items::replace_all;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let xs = black_box(&[
        "ooh", "la", "la", "oooooh", "lah", "lol", "this", "is", "so", "gooood",
    ]);
    let find = black_box("la");
    let replace = black_box("baby");
    bencher.iter(|| replace_all(xs, find, replace));
}
