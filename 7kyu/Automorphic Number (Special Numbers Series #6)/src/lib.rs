//! <https://www.codewars.com/kata/5a58d889880385c2f40000aa/train/rust>

const AUTOMORPHIC_NUMBERS: [u64; 28] = [
    0,
    1,
    5,
    6,
    25,
    76,
    376,
    625,
    9376,
    90625,
    109376,
    890625,
    2890625,
    7109376,
    12890625,
    87109376,
    212890625,
    787109376,
    1787109376,
    8212890625,
    18212890625,
    81787109376,
    918212890625,
    9918212890625,
    40081787109376,
    59918212890625,
    259918212890625,
    740081787109376,
];

pub fn automorphic(n: u64) -> String {
    if AUTOMORPHIC_NUMBERS.contains(&n) {
        "Automorphic"
    } else {
        "Not!!"
    }
    .into()
}
