//! <https://www.codewars.com/kata/57a6633153ba33189e000074/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::{hash_map::Entry, HashMap};
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    if sip.is_ascii() {
        return ordered_count_bytes(sip.as_bytes());
    }

    let mut counts = FxHashMap::with_capacity_and_hasher(sip.len(), Default::default());
    let mut res = Vec::<(char, i32)>::with_capacity(sip.len());

    for ch in sip.chars() {
        if counts.len() == counts.capacity() {
            unsafe { unreachable_unchecked() };
        }
        match counts.entry(ch) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                unsafe {
                    let len = res.len();
                    res.set_len(len + 1);
                    res.get_unchecked_mut(len).0 = ch;
                }
                e.insert(1);
            }
        }
    }

    for x in &mut res {
        x.1 = unsafe { *counts.get(&x.0).unwrap_unchecked() };
    }

    res
}

fn ordered_count_bytes(sip: &[u8]) -> Vec<(char, i32)> {
    let mut counts = [0; 256];
    let mut res = Vec::<(char, i32)>::with_capacity(256);

    for &b in sip {
        if counts[b as usize] != 0 {
            counts[b as usize] += 1;
        } else {
            unsafe {
                let len = res.len();
                res.set_len(len + 1);
                res.get_unchecked_mut(len).0 = b as char;
            }
            counts[b as usize] = 1;
        }
    }

    for x in &mut res {
        x.1 = counts[x.0 as u8 as usize];
    }

    res
}
