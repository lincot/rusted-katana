#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use toleetspeak::to_leet_speak;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| to_leet_speak(black_box("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG")));
}
