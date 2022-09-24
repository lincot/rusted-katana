#![no_std]
#![feature(test)]

extern crate test;
use scheduling_shortest_job_first_or_sjf::sjf;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let jobs = black_box(&[
        16, 30, 98, 14, 8, 88, 73, 72, 86, 62, 22, 48, 78, 81, 38, 47, 87, 29, 17, 47, 59, 96, 27,
        41, 44, 98, 93, 36, 52, 54, 7, 66, 60, 0, 87, 28, 49, 62, 68, 51, 18, 6, 34, 62, 71, 92,
        27, 39, 96, 52, 10, 27, 26, 27, 86, 19, 81, 96, 25, 26, 27, 89, 41, 12, 7, 54, 9, 18, 26,
        14, 8, 52, 56, 81, 27, 82, 49, 8, 65, 47, 72, 60, 40, 68, 83, 22, 48, 38, 51, 50, 85, 18,
        48, 85, 43, 38, 100, 1, 92, 12,
    ]);
    let index = black_box(50);
    bencher.iter(|| sjf(jobs, index));
}
