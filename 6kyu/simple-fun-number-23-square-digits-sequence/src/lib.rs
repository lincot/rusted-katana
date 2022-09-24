//! <https://www.codewars.com/kata/5886d65e427c27afeb0000c1/train/rust>

#![no_std]

pub fn square_digits_sequence(a0: u32) -> usize {
    assert!(a0 <= 650);

    let mut an = a0;
    let mut seen = heapless::FnvIndexSet::<_, 32>::new();

    loop {
        if seen.len() == seen.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        unsafe { seen.insert(an).unwrap_unchecked() };

        let mut a_next = 0;
        while an != 0 {
            a_next += (an % 10).pow(2);
            an /= 10;
        }

        if seen.contains(&a_next) {
            break;
        }
        an = a_next;
    }

    seen.len() + 1
}
