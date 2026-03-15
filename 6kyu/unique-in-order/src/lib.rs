//! <https://www.codewars.com/kata/54e6533c92449cc251001667/train/rust>

#![expect(incomplete_features)]
#![feature(trusted_len)]
#![feature(specialization)]

use core::{fmt::Debug, iter::TrustedLen};
use unchecked_std::prelude::*;

pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + Debug,
{
    let mut sequence = sequence.into_iter();
    let size_hint = sequence.size_hint();
    let mut res = Vec::with_capacity(size_hint.1.unwrap_or(size_hint.0));

    let Some(first) = sequence.next() else {
        return res;
    };
    unsafe { PushOrPushUnchecked::<T::IntoIter>::push_or_push_unchecked(&mut res, first) };
    let mut last_ptr: *const _ = res.last().unwrap();
    for x in sequence {
        if &x != unsafe { &*last_ptr } {
            unsafe { PushOrPushUnchecked::<T::IntoIter>::push_or_push_unchecked(&mut res, x) };
            last_ptr = res.last().unwrap();
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
