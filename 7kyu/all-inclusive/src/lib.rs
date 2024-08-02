//! <https://www.codewars.com/kata/5700c9acc1555755be00027e/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::HashSet;
use rustc_hash::FxHasher;
use unchecked_std::prelude::*;

type FxHashSet<K> = HashSet<K, BuildHasherDefault<FxHasher>>;

pub fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    let mut set = FxHashSet::with_capacity_and_hasher(arr.len(), Default::default());
    for s in arr.into_iter().filter(|s| s.len() == strng.len()) {
        if set.len() == set.capacity() {
            unsafe { unreachable_unchecked() };
        }
        set.insert(s);
    }

    let mut twice = String::with_capacity(2 * strng.len());
    unsafe {
        twice.push_str_unchecked(strng);
        twice.push_str_unchecked(strng);
    }

    strng.char_indices().all(|(offset, _)| {
        set.contains(&unsafe { twice.get_unchecked(offset..strng.len() + offset) })
    })
}
