//! <https://www.codewars.com/kata/5700c9acc1555755be00027e/train/rust>

use rustc_hash::FxHashSet;

pub fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    let mut set = FxHashSet::with_capacity_and_hasher(arr.len(), Default::default());
    set.extend(arr);

    let twice = strng.repeat(2);

    (0..strng.len())
        .all(|offset| set.contains(&unsafe { twice.get_unchecked(offset..strng.len() + offset) }))
}
