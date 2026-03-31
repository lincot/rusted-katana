#![allow(dead_code)]

use std::{collections::HashMap, sync::LazyLock};

pub static NATO: LazyLock<HashMap<char, &'static str>> = LazyLock::new(|| {
    [
        ('A', "Alfa"),
        ('B', "Bravo"),
        ('C', "Charlie"),
        ('D', "Delta"),
        ('E', "Echo"),
        ('F', "Foxtrot"),
        ('G', "Golf"),
        ('H', "Hotel"),
        ('I', "India"),
        ('J', "Juliett"),
        ('K', "Kilo"),
        ('L', "Lima"),
        ('M', "Mike"),
        ('N', "November"),
        ('O', "Oscar"),
        ('P', "Papa"),
        ('Q', "Quebec"),
        ('R', "Romeo"),
        ('S', "Sierra"),
        ('T', "Tango"),
        ('U', "Uniform"),
        ('V', "Victor"),
        ('W', "Whiskey"),
        ('X', "Xray"),
        ('Y', "Yankee"),
        ('Z', "Zulu"),
    ]
    .iter()
    .copied()
    .collect()
});
