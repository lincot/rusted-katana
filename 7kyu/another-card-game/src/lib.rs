//! <https://www.codewars.com/kata/633874ed198a4c00286aa39d/train/rust>

#![no_std]

pub fn the_game(frank: &[u8; 4], sam: &[u8; 4], tom: &[u8; 4]) -> bool {
    [sam, tom]
        .into_iter()
        .all(|p| frank[2..].iter().zip(p).all(|(f, p)| f > p))
}
