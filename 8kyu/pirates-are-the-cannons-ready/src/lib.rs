//! <https://www.codewars.com/kata/5748a883eb737cab000022a6/train/rust>

use std::collections::HashMap;

pub fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    (if gunners.values().all(|s| s.bytes().next() == Some(b'a')) {
        "Fire!"
    } else {
        "Shiver me timbers!"
    })
    .into()
}
