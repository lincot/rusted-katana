//! <https://www.codewars.com/kata/5574835e3e404a0bed00001b/train/rust>

pub fn get_participants(handshakes: u32) -> u32 {
    if handshakes == 0 {
        return 0;
    }
    (((8 * handshakes + 1) as f64).sqrt() + 1.).mul_add(1. / 2., 1. - 1e-9) as _
}
