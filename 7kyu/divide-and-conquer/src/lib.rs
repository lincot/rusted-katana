//! <https://www.codewars.com/kata/57eaec5608fed543d6000021/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use either::Either;

pub fn div_con(arr: &[Either<i32, String>]) -> i32 {
    arr.iter()
        .map(|e| match e {
            Either::Left(x) => *x,
            Either::Right(x) => -x.parse::<i32>().unwrap(),
        })
        .sum()
}
