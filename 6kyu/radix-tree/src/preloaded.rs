#![expect(clippy::from_iter_instead_of_collect)]

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RadixTree(pub HashMap<String, Self>);

/// These are convenience methods used for tests creation, but you
/// are free to use them yourself if you find a reason to.
impl RadixTree {
    pub fn new<I: IntoIterator<Item = (&'static str, Self)>>(words: I) -> Self {
        Self(HashMap::from_iter(
            words.into_iter().map(|(k, v)| (String::from(k), v)),
        ))
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }
}
