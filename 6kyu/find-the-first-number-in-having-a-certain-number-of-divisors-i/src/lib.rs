//! <https://www.codewars.com/kata/5612ab201830eb000f0000c0/train/rust>

pub const fn find_min_num(num: u8) -> u32 {
    [
        0,
        1,
        2,
        4,
        6,
        16,
        12,
        64,
        24,
        36,
        48,
        1_024,
        60,
        4_096,
        192,
        144,
        120,
        65_536,
        180,
        262_144,
        240,
        576,
        3_072,
        4_194_304,
        360,
        1_296,
        12_288,
        900,
        960,
        268_435_456,
        720,
        1_073_741_824,
        840,
        9_216,
        196_608,
        5_184,
        1_260,
        0, // 68_719_476_736,
        786_432,
        36_864,
        1_680,
        0, // 1_099_511_627_776,
        2_880,
        0, // 4_398_046_511_104,
        15_360,
        3_600,
        12_582_912,
        0, // 70_368_744_177_664,
        2_520,
        46_656,
        6_480,
        589_824,
        61_440,
        0, // 4_503_599_627_370_496,
        6_300,
        82_944,
        6_720,
        2_359_296,
        805_306_368,
        0, // 288_230_376_151_711_744,
        5_040,
        0, // 1_152_921_504_606_846_976,
        3_221_225_472,
        14_400,
        7_560,
        331_776,
        46_080,
        0, // 73_786_976_294_838_206_464,
        983_040,
        37_748_736,
        25_920,
        0, // 1_180_591_620_717_411_303_424,
        10_080,
        0, // 4_722_366_482_869_645_213_696,
        0, // 206_158_430_208,
        32_400,
        3_932_160,
        746_496,
        184_320,
        0, // 302_231_454_903_657_293_676_544,
    ][num as usize]
}
