//! <https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/rust>

use char_to_lower::to_lower;
use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::HashMap;
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn count_duplicates(text: &str) -> u32 {
    let mut map = FxHashMap::with_capacity_and_hasher(text.len(), Default::default());

    let mut latin_map = [0usize; 26];
    let mut digit_map = [0usize; 10];

    for ch in text.chars() {
        if ch.is_ascii_lowercase() {
            latin_map[(ch as u8 - b'a') as usize] += 1;
        } else if ch.is_ascii_uppercase() {
            latin_map[(ch as u8 - b'A') as usize] += 1;
        } else if ch.is_ascii_digit() {
            digit_map[(ch as u8 - b'0') as usize] += 1;
        } else {
            if map.len() == map.capacity() {
                unsafe { unreachable_unchecked() };
            }
            map.entry(to_lower(ch))
                .and_modify(|x| *x = true)
                .or_insert(false);
        }
    }

    map.values().filter(|&&x| x).count() as u32
        + latin_map.iter().filter(|&&count| count > 1).count() as u32
        + digit_map.iter().filter(|&&count| count > 1).count() as u32
}
