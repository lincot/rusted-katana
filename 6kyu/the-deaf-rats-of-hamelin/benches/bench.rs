#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use the_deaf_rats_of_hamelin::count_deaf_rats;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_deaf_rats(black_box("~O~O~O~OP~O~OO~")));
}
