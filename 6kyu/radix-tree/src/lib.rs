//! <https://www.codewars.com/kata/5c9d62cbf1613a001af5b067/train/rust>

#![feature(slice_swap_unchecked)]

use std::collections::HashMap;

pub use preloaded::RadixTree;

mod preloaded;

pub fn radix_tree(words: &[&str]) -> RadixTree {
    let mut words = words.to_vec().into_boxed_slice();
    unsafe { radix_tree_(&mut words, 0) }
}

unsafe fn radix_tree_(mut words: &mut [&str], ignored_bytes: usize) -> RadixTree {
    let mut res = HashMap::new();

    while let Some(base_word) = words.iter().next() {
        let base_word = unsafe { base_word.get_unchecked(ignored_bytes..) };
        let Some(first_ch) = base_word.chars().next() else {
            words = &mut words[1..];
            continue;
        };
        let mut min_cmn_pref = base_word.len();
        let mut group_len = 1;

        for i in 1..words.len() {
            let second_word = unsafe { words[i].get_unchecked(ignored_bytes..) };
            if !second_word.starts_with(first_ch) {
                continue;
            }

            min_cmn_pref = min_cmn_pref.min(second_word.len());
            for (i, (b1, b2)) in unsafe { base_word.get_unchecked(..min_cmn_pref) }
                .bytes()
                .zip(second_word.bytes())
                .enumerate()
            {
                if b1 != b2 {
                    min_cmn_pref = i;
                    break;
                }
            }

            unsafe { words.swap_unchecked(i, group_len) };
            group_len += 1;
        }

        while !base_word.is_char_boundary(min_cmn_pref) {
            min_cmn_pref -= 1;
        }

        unsafe {
            let (group, rest_words) = words.split_at_mut_unchecked(group_len);
            res.insert(
                base_word.get_unchecked(..min_cmn_pref).into(),
                radix_tree_(group, ignored_bytes + min_cmn_pref),
            );
            words = rest_words;
        }
    }

    RadixTree(res)
}
