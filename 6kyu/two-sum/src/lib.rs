//! <https://www.codewars.com/kata/52c31f8e6605bcc646000082/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::HashMap;
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    let mut map = FxHashMap::with_capacity_and_hasher(numbers.len(), Default::default());
    for (i, &x) in numbers.iter().enumerate() {
        if let Some(&j) = map.get(&(target - x)) {
            return (i, j);
        }
        if map.len() == map.capacity() {
            unsafe { unreachable_unchecked() };
        }
        map.insert(x, i);
    }
    panic!();
}
