//! <https://www.codewars.com/kata/69e4c22bd10a363dd5b75a7f/train/rust>

use digital::prelude::*;
use num_integer::Integer;
use unchecked_std::prelude::*;

pub fn ratio_combinator(ratios: &[&str]) -> String {
    assert!(ratios.len() <= MAX_LEN);

    let mut r = 1;
    let mut ratios_parsed: Vec<u128> = ratios
        .iter()
        .map(|ratio| {
            let (a, b) = reduce_fraction(parse_ratio(ratio));
            r *= a as u128;
            pack(a, b)
        })
        .collect();

    let first = r;
    let mut gcd = r;
    for slot in &mut ratios_parsed {
        let (a, b) = unpack(*slot);
        r /= a as u128;
        r *= b as u128;
        gcd = gcd.gcd(&r);
        *slot = r;
    }

    let mut res = String::with_capacity(get_capacity(ratios.len()));
    unsafe { res.write_int_unchecked(first / gcd) };
    for r in ratios_parsed {
        unsafe { res.push_unchecked(':') };
        unsafe { res.write_int_unchecked(r / gcd) };
    }
    res
}

const fn pack(a: u32, b: u32) -> u128 {
    ((a as u128) << 32) | (b as u128)
}

const fn unpack(v: u128) -> (u32, u32) {
    ((v >> 32) as u32, v as u32)
}

const fn parse_ratio(ratio: &str) -> (u32, u32) {
    let ratio = ratio.as_bytes();

    let (mut a, mut b) = (0, 0);

    let mut i = 0;
    while i < ratio.len() {
        if ratio[i] == b':' {
            i += 1;
            break;
        }

        a *= 10;
        a += (ratio[i] - b'0') as u32;
        i += 1;
    }

    while i < ratio.len() {
        b *= 10;
        b += (ratio[i] - b'0') as u32;
        i += 1;
    }

    (a, b)
}

fn reduce_fraction(fraction: (u32, u32)) -> (u32, u32) {
    let gcd = fraction.0.gcd(&fraction.1);
    (fraction.0 / gcd, fraction.1 / gcd)
}

const fn get_capacity(len: usize) -> usize {
    (len + 1) * (u128::MAX_LEN_BASE10 + 1)
}

#[cfg(any(target_pointer_width = "64", test))]
const MAX_LEN_64: usize = 230_584_300_921_369_394;
#[cfg(any(target_pointer_width = "32", test))]
const MAX_LEN_32: usize = 53_687_090;
#[cfg(any(target_pointer_width = "16", test))]
const MAX_LEN_16: usize = 818;

#[cfg(target_pointer_width = "64")]
const MAX_LEN: usize = MAX_LEN_64;
#[cfg(target_pointer_width = "32")]
const MAX_LEN: usize = MAX_LEN_32;
#[cfg(target_pointer_width = "16")]
const MAX_LEN: usize = MAX_LEN_16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_len() {
        assert!(i64::try_from(get_capacity(MAX_LEN_64 as _)).is_ok());
        assert!(i64::try_from(get_capacity((MAX_LEN_64 + 1) as _)).is_err());

        assert!(i32::try_from(get_capacity(MAX_LEN_32 as _)).is_ok());
        assert!(i32::try_from(get_capacity((MAX_LEN_32 + 1) as _)).is_err());

        assert!(i16::try_from(get_capacity(MAX_LEN_16 as _)).is_ok());
        assert!(i16::try_from(get_capacity((MAX_LEN_16 + 1) as _)).is_err());
    }
}
