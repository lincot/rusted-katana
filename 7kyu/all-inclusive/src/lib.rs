//! <https://www.codewars.com/kata/5700c9acc1555755be00027e/train/rust>

use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

pub fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    let mut set = FxHashMap::with_capacity_and_hasher(arr.len(), Default::default());
    for s in arr.into_iter().filter(|s| s.len() == strng.len()) {
        if set.len() == set.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        if let Entry::Vacant(e) = set.entry(s) {
            e.insert(());
        }
    }

    let twice = strng.repeat(2);

    (0..strng.len()).all(|offset| {
        set.contains_key(&unsafe { twice.get_unchecked(offset..strng.len() + offset) })
    })
}
