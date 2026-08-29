//! <https://www.codewars.com/kata/585d8c8a28bc7403ea0000c3/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};

use char_to_lower::to_lower;
use hashbrown::HashSet;
use rustc_hash::FxHasher;

type FxHashSet<K> = HashSet<K, BuildHasherDefault<FxHasher>>;

pub fn find_uniq<'a>(arr: &'a [&str]) -> &'a str {
    let &[first, second, third, ..] = arr else {
        panic!();
    };
    let [first_c, second_c, third_c] = [first, second, third].map(lower_set);
    if first_c != second_c {
        return if first_c == third_c { second } else { first };
    }

    arr[2..]
        .iter()
        .copied()
        .find(|&x| lower_set(x) != first_c)
        .unwrap()
}

fn lower_set(s: &str) -> FxHashSet<char> {
    let mut res = FxHashSet::with_capacity_and_hasher(s.len(), Default::default());
    for ch in s.chars() {
        if !ch.is_whitespace() {
            if res.len() == res.capacity() {
                unsafe { unreachable_unchecked() };
            }
            res.insert(to_lower(ch));
        }
    }
    res
}
