//! <https://www.codewars.com/kata/5a262cfb8f27f217f700000b/train/rust>

use rustc_hash::FxHashSet;

pub fn solve(a: &str, b: &str) -> String {
    let mut map_a = FxHashSet::with_capacity_and_hasher(a.len(), Default::default());
    map_a.extend(a.chars());
    let mut map_b = FxHashSet::with_capacity_and_hasher(b.len(), Default::default());
    map_b.extend(b.chars());

    let mut res = String::with_capacity(a.len() + b.len());
    res.extend(a.chars().filter(|c| !map_b.contains(c)));
    res.extend(b.chars().filter(|c| !map_a.contains(c)));
    res
}
