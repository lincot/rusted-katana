#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&[
        409, 264, 748, 3, 52, 675, 796, 563, 216, 750, 868, 424, 619, 591, 41, 689, 107, 133, 575,
        968, 257, 532, 941, 261, 32, 371, 271, 741, 284, 684, 197, 670, 301, 285, 679, 491, 223,
        345, 331, 628, 793, 677, 543, 14, 262, 463, 474, 286, 995, 180, 403, 743, 804, 12, 308,
        606, 305, 506, 897, 984, 835, 369, 829, 132, 611, 548, 996, 193, 670, 555, 676, 8, 838,
        130, 748, 910, 380, 201, 168, 966, 205, 507, 193, 177, 902, 527, 726, 339, 374, 959, 677,
        357, 211, 724, 940, 370, 248, 520, 707, 6, 205,
    ]);
    let n = black_box(234);
    bencher.iter(|| solution::split_and_add(arr, n))
}
