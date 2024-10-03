//! <https://www.codewars.com/kata/5872637c2eefcb1216000081/train/rust>

use hashbrown::HashSet;

pub fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let mut res = "a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string();
    for set in input_sets {
        for ch in set {
            if ch.is_ascii_lowercase() {
                unsafe {
                    *res.as_mut_vec()
                        .get_unchecked_mut((2 * (ch as u8 - b'a')) as usize) = b'_';
                }
            }
        }
    }
    res
}
