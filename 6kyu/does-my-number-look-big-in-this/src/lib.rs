//! <https://www.codewars.com/kata/5287e858c6b5a9678200083c/train/rust>

pub fn narcissistic(num: u64) -> bool {
    NARCISSISTIC_NUMBERS.binary_search(&num).is_ok()
}

const NARCISSISTIC_NUMBERS: &[u64] = &[
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    153,
    370,
    371,
    407,
    1_634,
    8_208,
    9_474,
    54_748,
    92_727,
    93_084,
    548_834,
    1_741_725,
    4_210_818,
    9_800_817,
    9_926_315,
    24_678_050,
    24_678_051,
    88_593_477,
    146_511_208,
    472_335_975,
    534_494_836,
    912_985_153,
    4_679_307_774,
    32_164_049_650,
    32_164_049_651,
    40_028_394_225,
    42_678_290_603,
    44_708_635_679,
    49_388_550_606,
    82_693_916_578,
    94_204_591_914,
    28_116_440_335_967,
    4_338_281_769_391_370,
    4_338_281_769_391_371,
    21_897_142_587_612_075,
    35_641_594_208_964_132,
    35_875_699_062_250_035,
    1_517_841_543_307_505_039,
    3_289_582_984_443_187_032,
    4_498_128_791_164_624_869,
    4_929_273_885_928_088_826,
];
