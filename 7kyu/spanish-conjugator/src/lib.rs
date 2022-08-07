//! <https://www.codewars.com/kata/5a81b78d4a6b344638000183/train/rust>

use my_prelude::prelude::*;
use std::collections::HashMap;

pub fn conjugate(verb: &str) -> HashMap<String, Vec<String>> {
    assert!(verb.len() >= 2);

    let mut base = Vec::with_capacity(verb.len() + 2);
    unsafe { base.extend_from_slice_unchecked(&verb.as_bytes()[..verb.len() - 2]) };
    let base = unsafe { String::from_utf8_unchecked(base) };

    let forms = match verb.as_bytes()[verb.len() - 2] {
        b'a' => {
            vec![
                base.clone() + "o",
                base.clone() + "as",
                base.clone() + "a",
                base.clone() + "amos",
                base.clone() + "áis",
                base + "an",
            ]
        }
        b'e' => {
            vec![
                base.clone() + "o",
                base.clone() + "es",
                base.clone() + "e",
                base.clone() + "emos",
                base.clone() + "éis",
                base + "en",
            ]
        }
        b'i' => {
            vec![
                base.clone() + "o",
                base.clone() + "es",
                base.clone() + "e",
                base.clone() + "imos",
                base.clone() + "ís",
                base + "en",
            ]
        }
        _ => panic!(),
    };

    HashMap::from([(verb.into(), forms)])
}
