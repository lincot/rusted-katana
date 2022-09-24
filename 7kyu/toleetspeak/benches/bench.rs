#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use toleetspeak::to_leet_speak;

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");
    bencher.iter(|| to_leet_speak(s));
}
