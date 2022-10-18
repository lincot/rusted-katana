#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use count_of_positives_slash_sum_of_negatives::count_positives_sum_negatives;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_positives_sum_negatives(black_box(vec![
            990, 366, 858, -867, 66, -647, 68, -949, 854, 831, 242, 898, -356, -694, 124, -995,
            -414, 488, -197, 996, -584, -353, 71, -640, -752, 935, -3, 797, 173, 714, -659, 947,
            -439, 711, 131, 699, -798, 503, 353, -961, 968, 869, -826, 461, -3, 312, 701, -209,
            301, -382, 790, -373, -668, 777, -386, -180, -343, 285, -740, -46, -205, 564, -883,
            -733, -942, 295, -881, 505, 273, 56, -797, -207, 142, -874, 210, -100, 364, 457, 225,
            -410, -391, -944, -509, 78, 876, 607, 194, 396, 951, -323, 287, 700, -636, -934, -106,
            237, 431, 98, -591, -574,
        ]))
    });
}
