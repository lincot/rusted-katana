#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

fn get_vecs() -> Vec<Vec<i32>> {
    vec![
        vec![-1, -8, 18, 7, -3],
        vec![7, -8, -18, -7],
        vec![20, 19, -13, 11, 10],
        vec![0, 9, 17],
        vec![-5, 18, 7, -16, -16],
    ]
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let vecs = get_vecs();
    let vecs = black_box(&vecs);

    bencher.iter(|| solution::solve(vecs))
}
