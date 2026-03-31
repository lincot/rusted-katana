//! <https://www.codewars.com/kata/635fc0497dadea0030cb7936/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};

use hashbrown::HashSet;
use rustc_hash::FxHasher;

type FxHashSet<K> = HashSet<K, BuildHasherDefault<FxHasher>>;

pub fn count_different_matrices(matrices: &[[i16; 4]]) -> usize {
    let mut set = FxHashSet::with_capacity_and_hasher(matrices.len(), Default::default());
    let mut res = 0;

    for &[a, b, c, d] in matrices {
        let min = [a, b, c, d]
            .min([c, a, d, b])
            .min([d, c, b, a])
            .min([b, d, a, c]);
        if set.len() == set.capacity() {
            unsafe { unreachable_unchecked() };
        }
        if !set.insert(encode_matrix(min)) {
            continue;
        }
        res += 1;
    }

    res
}

const fn encode_matrix([a, b, c, d]: [i16; 4]) -> u16 {
    ((a as u16) << 12) + ((b as u16) << 8) + ((c as u16) << 4) + d as u16
}
