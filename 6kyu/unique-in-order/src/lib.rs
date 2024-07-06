//! <https://www.codewars.com/kata/54e6533c92449cc251001667/train/rust>

#![allow(incomplete_features)]
#![feature(trusted_len)]
#![feature(specialization)]

use core::{fmt::Debug, hint::unreachable_unchecked, iter::TrustedLen};
use unchecked_std::prelude::*;

pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + Debug,
{
    let mut sequence = sequence.into_iter();
    let mut res = sequence
        .size_hint()
        .1
        .map_or_else(Vec::new, Vec::with_capacity);

    let Some(first) = sequence.next() else {
        return res;
    };
    unsafe { PushOrPushUnchecked::<T::IntoIter>::push_or_push_unchecked(&mut res, first) };
    for x in sequence {
        if res.is_empty() {
            unsafe { unreachable_unchecked() };
        }
        if &x != res.last().unwrap() {
            unsafe { PushOrPushUnchecked::<T::IntoIter>::push_or_push_unchecked(&mut res, x) };
        }
    }
    res
}

trait PushOrPushUnchecked<I: Iterator> {
    unsafe fn push_or_push_unchecked(&mut self, value: I::Item);
}

impl<I: Iterator> PushOrPushUnchecked<I> for Vec<I::Item> {
    default unsafe fn push_or_push_unchecked(&mut self, value: I::Item) {
        self.push(value);
    }
}

impl<I: TrustedLen> PushOrPushUnchecked<I> for Vec<I::Item> {
    unsafe fn push_or_push_unchecked(&mut self, value: I::Item) {
        unsafe { self.push_unchecked(value) };
    }
}
