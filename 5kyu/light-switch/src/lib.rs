//! <https://www.codewars.com/kata/63306fdffa185d004a987b8e/train/rust>

pub fn light_switch(n: u8, corresponding_lights_list: &Vec<Vec<u8>>) -> bool {
    let mut basis = [0; 32];
    for switch in corresponding_lights_list {
        let mut vec = switch.iter().map(|i| 1 << i).sum::<u32>();
        loop {
            let b = bit_length(vec);
            if b == 0 || basis[b - 1] == 0 {
                break;
            }
            vec ^= basis[b - 1];
        }
        if vec != 0 {
            basis[bit_length(vec) - 1] = vec;
        }
    }

    let mut target = (1 << n) as u32 - 1;
    loop {
        let b = bit_length(target);
        if b == 0 || basis[b - 1] == 0 {
            break;
        }
        target ^= basis[b - 1];
    }

    target == 0
}

const fn bit_length(n: u32) -> usize {
    32 - n.leading_zeros() as usize
}
