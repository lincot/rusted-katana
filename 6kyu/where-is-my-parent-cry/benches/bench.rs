#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use where_is_my_parent_cry::find_children;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        find_children(black_box(
            "abBAAaaaaZazzzCbcBcbaAxXfuUuuFxXfuUuuFxXfuUuuFLlllll",
        ))
    });
}
