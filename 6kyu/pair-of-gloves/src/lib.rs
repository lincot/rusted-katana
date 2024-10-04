//! <https://www.codewars.com/kata/58235a167a8cb37e1a0000db/train/rust>

use unchecked_std::prelude::*;

pub fn number_of_pairs(gloves: &[&str]) -> u32 {
    let mut counts = Vec::with_capacity(gloves.len());
    for glove in gloves {
        if let Some((_, count)) = counts.iter_mut().find(|&&mut (g, _)| g == glove) {
            *count += 1;
        } else {
            unsafe { counts.push_unchecked((glove, 1)) };
        }
    }
    counts.iter().map(|(_, count)| count / 2).sum()
}
