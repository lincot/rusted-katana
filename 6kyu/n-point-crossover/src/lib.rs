//! <https://www.codewars.com/kata/57339a5226196a7f90001bcf/train/rust>

use unchecked_std::prelude::*;

pub fn crossover(ns: &[usize], xs: &[u8], ys: &[u8]) -> (Vec<u8>, Vec<u8>) {
    assert!(xs.len() == ys.len());

    let mut ns = ns.to_vec();
    vqsort_rs::sort(&mut ns);
    assert!(ns.last().map_or(true, |&last| last < xs.len()));
    ns.dedup();

    unsafe {
        let mut new_xs = Vec::with_capacity(xs.len());
        new_xs.set_len(xs.len());
        let mut new_ys = Vec::with_capacity(ys.len());
        new_ys.set_len(ys.len());

        let mut prev_point = 0;
        let mut is_crossover = false;
        for point in ns {
            let (src_xs, src_ys) = if is_crossover { (ys, xs) } else { (xs, ys) };
            new_xs
                .get_unchecked_mut(prev_point..point)
                .copy_from_slice_unchecked(src_xs.get_unchecked(prev_point..point));
            new_ys
                .get_unchecked_mut(prev_point..point)
                .copy_from_slice_unchecked(src_ys.get_unchecked(prev_point..point));

            prev_point = point;
            is_crossover = !is_crossover;
        }

        let (src_xs, src_ys) = if is_crossover { (ys, xs) } else { (xs, ys) };
        new_xs
            .get_unchecked_mut(prev_point..)
            .copy_from_slice_unchecked(src_xs.get_unchecked(prev_point..));
        new_ys
            .get_unchecked_mut(prev_point..)
            .copy_from_slice_unchecked(src_ys.get_unchecked(prev_point..));

        (new_xs, new_ys)
    }
}
