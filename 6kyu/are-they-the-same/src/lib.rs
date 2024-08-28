//! <https://www.codewars.com/kata/550498447451fbbd7600041c/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::HashMap;
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut a_squared = FxHashMap::with_capacity_and_hasher(a.len(), Default::default());
    for num in a {
        if a_squared.len() == a_squared.capacity() {
            unsafe { unreachable_unchecked() };
        }
        a_squared
            .entry(num * num)
            .and_modify(|count| *count += 1)
            .or_insert(1usize);
    }
    for num in b {
        match a_squared.get_mut(&num) {
            Some(count) if *count != 0 => *count -= 1,
            _ => return false,
        }
    }
    true
}
