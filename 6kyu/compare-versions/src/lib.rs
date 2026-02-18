//! <https://www.codewars.com/kata/53b138b3b987275b46000115/train/rust>

use core::cmp::Ordering;

pub fn compare_versions(mut version1: &str, mut version2: &str) -> bool {
    loop {
        let dot_pos1 = version1
            .as_bytes()
            .iter()
            .position(|&x| x == b'.')
            .unwrap_or(version1.len());
        let subversion1 = parse_fake_int(unsafe { version1.get_unchecked(..dot_pos1) });

        let dot_pos2 = version2
            .as_bytes()
            .iter()
            .position(|&x| x == b'.')
            .unwrap_or(version2.len());
        let subversion2 = parse_fake_int(unsafe { version2.get_unchecked(..dot_pos2) });

        match subversion1.cmp(&subversion2) {
            Ordering::Less => return false,
            Ordering::Greater => return true,
            Ordering::Equal => {}
        }

        if dot_pos2 == version2.len() {
            return true;
        }
        version2 = unsafe { version2.get_unchecked(dot_pos2 + 1..) };

        if dot_pos1 == version1.len() {
            return false;
        }
        version1 = unsafe { version1.get_unchecked(dot_pos1 + 1..) };
    }
}

fn parse_fake_int(s: &str) -> u32 {
    let mut res = 0;
    for &b in s.as_bytes() {
        res <<= 6;
        res += b as u32;
    }
    res
}
