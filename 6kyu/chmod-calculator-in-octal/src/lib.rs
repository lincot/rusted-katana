//! <https://www.codewars.com/kata/57f4ccf0ab9a91c3d5000054/train/rust>

use core::mem::MaybeUninit;
use std::collections::HashMap;

pub fn chmod_calculator(perm: &HashMap<&str, &str>) -> String {
    let mut res = Vec::with_capacity(3);
    for (i, entity) in ["user", "group", "other"].iter().enumerate() {
        let n = perm.get(entity).map_or(b'0', |s| {
            let s = s.as_bytes();
            assert!(s.len() == 3);
            let mut num = b'0';
            if s[0] != b'-' {
                num += 4;
            }
            if s[1] != b'-' {
                num += 2;
            }
            if s[2] != b'-' {
                num += 1;
            }
            num
        });
        unsafe { *res.spare_capacity_mut().get_unchecked_mut(i) = MaybeUninit::new(n) };
    }
    unsafe {
        res.set_len(3);
        String::from_utf8_unchecked(res)
    }
}
