//! <https://www.codewars.com/kata/6040b781e50db7000ab35125/train/rust>

use core::{mem::replace, ops::Sub};

pub fn delta<I: IntoIterator<Item = T>, T: Sub<Output = T> + Copy>(
    values: I,
    level: usize,
) -> DeltaIterator<I::IntoIter, T> {
    assert!((1..=50).contains(&level));
    let mut values = values.into_iter();
    let mut deltas = heapless::Vec::new();
    for _ in 0..level {
        let e = deltas
            .iter_mut()
            .fold(values.next().unwrap(), |a, x| a - replace(x, a));
        unsafe { deltas.push_unchecked(e) };
    }
    DeltaIterator { values, deltas }
}

pub struct DeltaIterator<I, T> {
    values: I,
    deltas: heapless::Vec<T, 50>,
}

impl<I: Iterator<Item = T>, T: Sub<Output = T> + Copy> Iterator for DeltaIterator<I, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.values
            .next()
            .map(|n| self.deltas.iter_mut().fold(n, |a, x| a - replace(x, a)))
    }
}
