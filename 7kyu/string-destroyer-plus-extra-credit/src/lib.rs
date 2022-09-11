//! <https://www.codewars.com/kata/5872637c2eefcb1216000081/train/rust>

use std::collections::HashSet;

pub fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let mut res = "a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string();
    for set in input_sets {
        for c in set {
            let c = (c as u8).wrapping_sub(b'a');
            if c < b'z' - b'a' + 1 {
                unsafe { res.as_mut_vec()[(2 * c) as usize] = b'_' };
            }
        }
    }
    res
}
