#![feature(test)]

extern crate test;
use grasshopper_personalized_message::greet;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(greet(black_box("guest"), black_box("owner")));
        black_box(greet(black_box("owner"), black_box("owner")));
    });
}
