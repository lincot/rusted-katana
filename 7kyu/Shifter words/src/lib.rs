//! <https://www.codewars.com/kata/603b2bb1c7646d000f900083/train/rust>

use rustc_hash::FxHashSet;

pub fn shifter(s: &str) -> usize {
    // arbitraty capacity
    let cap = s.len() / 4;
    let mut set = FxHashSet::with_capacity_and_hasher(cap, Default::default());
    set.extend(
        s.split_ascii_whitespace()
            .filter(|word| word.bytes().all(|b| b"HINOSXZMW".contains(&b))),
    );
    set.len()
}
