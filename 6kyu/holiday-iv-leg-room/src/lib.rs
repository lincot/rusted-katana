//! <https://www.codewars.com/kata/57e8ff073d1cb559280005de/train/rust>

pub fn leg_room(a: u32, b: &str) -> &'static str {
    let mut room = 0;
    for b in b.bytes() {
        if b == b'0' {
            return "Jackpot!";
        }
        if !b"aeiou".contains(&b) {
            room += 2;
        }
    }
    let leg_length = a * 55 / 100;
    if room > leg_length / 4 {
        "super comfy"
    } else if room > leg_length * 3 / 20 {
        "comfortable"
    } else {
        "ouch"
    }
}
