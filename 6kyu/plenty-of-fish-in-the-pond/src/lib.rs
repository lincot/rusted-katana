//! <https://www.codewars.com/kata/5904be220881cb68be00007d/train/rust>

pub fn fish(shoal: &str) -> u32 {
    let mut fish_counts = [0; 9];

    for fish in shoal.as_bytes() {
        if (b'1'..=b'9').contains(fish) {
            fish_counts[(fish - b'1') as usize] += 1;
        }
    }

    let mut total_size = 0;
    for ((fish_size, fish_count), size_needed) in (1..)
        .zip(fish_counts)
        .zip([0, 4, 12, 24, 40, 60, 84, 112, 144])
    {
        if total_size < size_needed {
            return fish_size - 1;
        }

        total_size += fish_size * fish_count;
    }

    (unsafe {
        ((2 * total_size + 1) as f64)
            .sqrt()
            .to_int_unchecked::<u32>()
    } - 1)
        / 2
        + 1
}
