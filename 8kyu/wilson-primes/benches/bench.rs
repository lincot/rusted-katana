// #![feature(test)]

// extern crate test;
// use test::{black_box, Bencher};
// use wilson_primes::am_i_wilson;

// #[bench]
// fn bench(bencher: &mut Bencher) {
//     bencher.iter(|| {
//         black_box(am_i_wilson(black_box(5)));
//         black_box(am_i_wilson(black_box(307)));
//         black_box(am_i_wilson(black_box(5971)));
//     });
// }

#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use wilson_primes::am_i_wilson;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(am_i_wilson(black_box(5)));
        black_box(am_i_wilson(black_box(307)));
        black_box(am_i_wilson(black_box(5971)));
    });
}
