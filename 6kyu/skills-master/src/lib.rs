//! <https://www.codewars.com/kata/648f2033d52f51608e06c458/train/rust>

use std::collections::HashSet;

pub fn count_skills(tree: &[usize], required: &HashSet<usize>) -> usize {
    let mut acquired = vec![false; tree.len()];
    let mut count = 0;
    for &(mut skill) in required {
        while !acquired[skill] {
            acquired[skill] = true;
            skill = tree[skill];
            count += 1;
        }
    }
    count
}
