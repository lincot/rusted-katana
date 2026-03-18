#![feature(test)]

extern crate test;
use oop_object_oriented_piracy::Ship;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Ship {
            draft: 60,
            crew: 20,
        })
        .is_worth_it()
    });
}
