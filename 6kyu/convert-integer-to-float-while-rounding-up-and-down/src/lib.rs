//! <https://www.codewars.com/kata/642eba25b8c5c20031058225/train/rust>

use num_bigint::{BigInt, Sign};

pub fn ifloor(n: &BigInt) -> f64 {
    if n.sign() == Sign::Plus {
        ifloor_pos(n)
    } else {
        -iceil_pos(n)
    }
}

pub fn iceil(n: &BigInt) -> f64 {
    if n.sign() == Sign::Plus {
        iceil_pos(n)
    } else {
        -ifloor_pos(n)
    }
}

fn ifloor_pos(n: &BigInt) -> f64 {
    let exp = (1 << (u64::BITS - f64::MANTISSA_DIGITS - 1)) - 2 + n.bits();
    let (exp, man) = if exp >= (1 << (u64::BITS - f64::MANTISSA_DIGITS)) - 1 {
        (
            (1 << (u64::BITS - f64::MANTISSA_DIGITS)) - 2,
            (1 << (f64::MANTISSA_DIGITS - 1)) - 1,
        )
    } else {
        let Some(last_digit) = n.iter_u64_digits().last() else {
            return 0.;
        };
        let shift = last_digit.leading_zeros() + 1;
        let mut man = if shift < u64::BITS {
            last_digit << shift
        } else {
            0
        };
        if let Some(penultimate_digit) = n.iter_u64_digits().rev().nth(1) {
            man += penultimate_digit >> (u64::BITS - shift);
        }
        man >>= u64::BITS - (f64::MANTISSA_DIGITS - 1);
        (exp, man)
    };
    f64::from_bits((exp << (f64::MANTISSA_DIGITS - 1)) + man)
}

fn iceil_pos(n: &BigInt) -> f64 {
    let Some(trailing_zeros) = n.trailing_zeros() else {
        return 0.;
    };
    let bits = n.bits();
    let exp = (1 << (u64::BITS - f64::MANTISSA_DIGITS - 1)) - 2 + bits;
    if exp >= (1 << (u64::BITS - f64::MANTISSA_DIGITS)) - 1 {
        f64::INFINITY
    } else {
        let floor = ifloor_pos(n).to_bits();
        let is_not_representable = (bits - trailing_zeros) > f64::MANTISSA_DIGITS as u64;
        f64::from_bits(floor + is_not_representable as u64)
    }
}
