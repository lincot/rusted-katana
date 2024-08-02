//! <https://www.codewars.com/kata/603b2bb1c7646d000f900083/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::HashSet;
use rustc_hash::FxHasher;

type FxHashSet<K> = HashSet<K, BuildHasherDefault<FxHasher>>;

pub fn shifter(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut set = FxHashSet::with_capacity_and_hasher(s.len() / 2 + 1, Default::default());

    for word in s
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|bytes| unsafe { core::str::from_utf8_unchecked(bytes) })
        .filter(|word| word.bytes().all(|b| b"HINOSXZMW".contains(&b)))
    {
        if set.len() == set.capacity() {
            unsafe { unreachable_unchecked() };
        }
        set.insert(word);
    }

    set.len()
}
