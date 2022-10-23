#![no_std]
#![feature(test)]

extern crate test;
use point_in_polygon_1::point_in_poly;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        point_in_poly(
            black_box(&[
                (3.183_847_2, 0.336_736_44),
                (4.275_992, 2.192_148_2),
                (2.332_238_7, 2.745_526_3),
                (1.807_235_2, 4.254_597),
                (0.252_980_65, 3.736_194_1),
                (-0.470_798_7, 4.331_998),
                (-2.246_675, 4.226_231_6),
                (-3.208_089_4, 3.210_599),
                (-3.915_342, 1.913_911_3),
                (-3.241_645, 0.249_400_05),
                (-3.234_219_6, -0.245_973_54),
                (-3.607_106, -2.261_355),
                (-3.937_360_3, -2.995_838_9),
                (-1.641_939_9, -3.117_805_7),
                (-0.899_784_9, -4.174_717),
                (1.006_968_5, -3.958_510_4),
                (1.180_444_6, -2.877_120_3),
                (2.696_522, -2.857_149_1),
                (3.422_392, -1.431_739_3),
                (4.615_703, -1.264_132_7),
                (3.792_268_3, 0.843_918_7),
            ]),
            black_box((-3.539_447, -2.693_076_6)),
        )
    });
}
