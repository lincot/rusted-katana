#![feature(test)]

extern crate test;
use hidden_cubic_numbers::is_sum_of_cubes;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const S: &str = "00 9026315 -827&() Once upon a midnight dreary, while100 I pondered, 9026315weak and weary -827&() aqdf&0#1xyz!22[153(777.777 QK29a45[&erui9026315";

    bencher.iter(|| is_sum_of_cubes(black_box(S)));
}
