#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};
use the_supermarket_queue::queue_time;

#[bench]
fn bench_1(bencher: &mut Bencher) {
    bench(bencher, 1);
}

#[bench]
fn bench_2(bencher: &mut Bencher) {
    bench(bencher, 2);
}

#[bench]
fn bench_3(bencher: &mut Bencher) {
    bench(bencher, 3);
}

#[bench]
fn bench_4(bencher: &mut Bencher) {
    bench(bencher, 4);
}

#[bench]
fn bench_5(bencher: &mut Bencher) {
    bench(bencher, 5);
}

#[bench]
fn bench_6(bencher: &mut Bencher) {
    bench(bencher, 6);
}

#[bench]
fn bench_7(bencher: &mut Bencher) {
    bench(bencher, 7);
}

#[bench]
fn bench_8(bencher: &mut Bencher) {
    bench(bencher, 8);
}

#[bench]
fn bench_9(bencher: &mut Bencher) {
    bench(bencher, 9);
}

#[bench]
fn bench_10(bencher: &mut Bencher) {
    bench(bencher, 10);
}

#[bench]
fn bench_11(bencher: &mut Bencher) {
    bench(bencher, 15);
}

#[bench]
fn bench_15(bencher: &mut Bencher) {
    bench(bencher, 15);
}

#[bench]
fn bench_16(bencher: &mut Bencher) {
    bench(bencher, 16);
}

fn bench(bencher: &mut Bencher, n: u32) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let customers: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| rng.gen_range(1..100));
    bencher.iter(|| queue_time(black_box(&customers), black_box(n)));
}
