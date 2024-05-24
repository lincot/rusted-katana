#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use scheduling_shortest_job_first_or_sjf::sjf;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let jobs: [_; 100] = array::from_fn(|_| rng.gen_range(1..100));
    bencher.iter(|| sjf(black_box(&jobs), black_box(50)));
}
