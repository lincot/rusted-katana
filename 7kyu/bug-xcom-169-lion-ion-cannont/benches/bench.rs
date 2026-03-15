#![feature(test)]

extern crate test;
use bug_xcom_169_lion_ion_cannont::release_ions;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut collected = Vec::with_capacity(50);
        release_ions(black_box(50), black_box(|| collected.push("H⁺")));
        collected
    });
}
