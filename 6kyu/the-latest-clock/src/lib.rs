//! <https://www.codewars.com/kata/58925dcb71f43f30cd00005f/train/rust>

pub fn latest_clock(a: u8, b: u8, c: u8, d: u8) -> String {
    let mut m = Multiset::default().add(a).add(b).add(c).add(d);
    let hh = m.pop(if m.count_le(5) >= 3 { 2 } else { 1 });
    let hl = m.pop(if hh == 2 { 3 } else { 9 });
    let mh = m.pop(5);
    let ml = m.pop(9);
    unsafe {
        String::from_utf8_unchecked(vec![
            b'0' + hh as u8,
            b'0' + hl as u8,
            b':',
            b'0' + mh as u8,
            b'0' + ml as u8,
        ])
    }
}

#[derive(Default, Clone, Copy)]
struct Multiset {
    m: u64,
}

impl Multiset {
    const fn add(self, d: u8) -> Self {
        Self {
            m: self.m | (self.m + (1 << (d * 4))),
        }
    }

    const fn count_le(self, max: u8) -> u32 {
        (self.m & ((1 << ((max + 1) * 4)) - 1)).count_ones()
    }

    const fn pop(&mut self, max: u8) -> u32 {
        let i = (self.m & ((1 << ((max + 1) * 4)) - 1)).bit_width() - 1;
        self.m &= !(1 << i);
        i / 4
    }
}
