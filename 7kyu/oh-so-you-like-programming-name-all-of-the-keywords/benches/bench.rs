#![feature(test)]

extern crate test;
use oh_so_you_like_programming_name_all_of_the_keywords::keywords;
use test::Bencher;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(keywords);
}
