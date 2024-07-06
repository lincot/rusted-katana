//! <https://www.codewars.com/kata/5626b561280a42ecc50000d1/train/rust>

pub fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let start = A032799.partition_point(|&x| x < a);
    let end = start + A032799[start..].partition_point(|&x| x <= b);
    unsafe { A032799.get_unchecked(start..end) }.into()
}

const A032799: [u64; 20] = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    89,
    135,
    175,
    518,
    598,
    1306,
    1676,
    2427,
    2_646_798,
    12_157_692_622_039_623_539,
];
