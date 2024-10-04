//! <https://www.codewars.com/kata/64f4ef596f222e004b877272/train/rust>

use core::hint::unreachable_unchecked;

pub fn rain_tacos(landscape: &str) -> String {
    if landscape.is_ascii() {
        let mut landscape = landscape.as_bytes().to_vec();
        rain_tacos_arr(&mut landscape, [b'T', b'A', b'C', b'O']);
        unsafe { String::from_utf8_unchecked(landscape) }
    } else {
        let mut landscape: Box<[_]> = landscape.chars().collect();
        rain_tacos_arr(&mut landscape, ['T', 'A', 'C', 'O']);
        landscape.iter().collect()
    }
}

fn rain_tacos_arr<T: Copy + From<u8> + Eq>(landscape: &mut [T], taco: [T; 4]) {
    let n = landscape
        .iter()
        .position(|&b| b == b'\n'.into())
        .unwrap_or(landscape.len());
    if n + 1 == 0 {
        unsafe { unreachable_unchecked() };
    }

    'cols: for j in 0..n {
        unsafe {
            let mut k = j;
            for i in 0..(landscape.len() + 1) / (n + 1) {
                if *landscape.get_unchecked(k) != b' '.into() {
                    if i != 0 {
                        let val_above = landscape.get_unchecked_mut(k - (n + 1));
                        if *val_above == b' '.into() {
                            *val_above = taco[j % taco.len()];
                        }
                    }
                    continue 'cols;
                }
                k += n + 1;
            }
            let len = landscape.len();
            *landscape.get_unchecked_mut(len - n + j) = taco[j % taco.len()];
        }
    }
}
