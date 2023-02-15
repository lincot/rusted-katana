//! <https://www.codewars.com/kata/6040b781e50db7000ab35125/train/rust>

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use core::{
    iter::empty,
    mem::{replace, MaybeUninit},
    ops::Sub,
};

pub fn delta<'a, I: IntoIterator<Item = T> + 'a, T: Sub<Output = T> + Copy + 'a>(
    values: I,
    level: usize,
) -> Box<dyn Iterator<Item = T> + 'a> {
    fn res<'a, I: Iterator<Item = T> + 'a, T: Sub<Output = T> + Copy + 'a, const LEVEL: usize>(
        mut values: I,
    ) -> Box<dyn Iterator<Item = T> + 'a> {
        let mut deltas = unsafe { MaybeUninit::<[_; LEVEL]>::uninit().assume_init() };
        for i in 0..LEVEL {
            let Some(next) = values.next() else {
                return Box::new(empty());
            };
            deltas[i] = deltas[..i].iter_mut().fold(next, |a, x| a - replace(x, a));
        }
        Box::new(DeltaIterator { values, deltas })
    }

    let values = values.into_iter();
    match level {
        1 => res::<_, _, 1>(values),
        2 => res::<_, _, 2>(values),
        3 => res::<_, _, 3>(values),
        4 => res::<_, _, 4>(values),
        5 => res::<_, _, 5>(values),
        6 => res::<_, _, 6>(values),
        7 => res::<_, _, 7>(values),
        8 => res::<_, _, 8>(values),
        9 => res::<_, _, 9>(values),
        10 => res::<_, _, 10>(values),
        _ => panic!(),
    }
}

pub struct DeltaIterator<I, T, const LEVEL: usize> {
    values: I,
    deltas: [T; LEVEL],
}

impl<I: Iterator<Item = T>, T: Sub<Output = T> + Copy, const LEVEL: usize> Iterator
    for DeltaIterator<I, T, LEVEL>
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.values
            .next()
            .map(|n| self.deltas.iter_mut().fold(n, |a, x| a - replace(x, a)))
    }
}
