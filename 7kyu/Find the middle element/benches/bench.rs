#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const INPUT_ARRAY: [i32; 3] = [1, 2, 3];

#[bench]
fn bench(bencher: &mut Bencher) {
    let input_array = black_box(INPUT_ARRAY);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::gimme(input_array));
        }
    })
}
