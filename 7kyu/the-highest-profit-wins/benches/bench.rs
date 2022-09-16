#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_highest_profit_wins::min_max;

#[bench]
fn bench(bencher: &mut Bencher) {
    let lst = black_box(&[
        919, -498, -225, 561, -60, 788, -529, -413, -862, -202, 362, 385, 295, -454, -999, 230,
        415, 467, 45, -985, 798, 694, 128, -939, 362, 625, -352, -520, -184, -283, 85, 172, -927,
        53, -554, 773, -968, 535, 10, -666, 634, -789, 442, 219, -470, 890, 947, -577, 726, 26,
        -179, -914, 956, 942, 446, 364, -600, -225, -605, -373, -737, 943, -585, 382, 268, -327,
        -958, 632, -602, -47, 301, 833, 959, 830, -191, 998, 885, 715, 150, 93, 630, 258, 665,
        -130, -215, -726, -169, -804, -931, 620, -352, -242, 285, 293, -363, -852, -921, -665, -29,
        -520,
    ]);
    bencher.iter(|| min_max(lst));
}