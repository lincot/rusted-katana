//! <https://www.codewars.com/kata/634ac4e77611b9f57dff456d/train/rust>

#![no_std]

use hashbrown::HashSet;

#[allow(clippy::missing_const_for_fn)]
pub fn keywords() -> HashSet<&'static str> {
    [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "union", "unsafe", "use", "where", "while",
    ]
    .into()
}