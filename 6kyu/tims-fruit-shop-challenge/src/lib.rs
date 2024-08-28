//! <https://www.codewars.com/kata/652643925c042100247fffc6/train/rust>

use unchecked_std::prelude::*;

pub fn fruit_pack(orders: &[&str]) -> Vec<(String, String, String)> {
    orders
        .iter()
        .map(|order| {
            let mut seen = [false; 26];
            let mut container_counts = heapless::Vec::<_, 26>::new();
            let mut count = 0;
            let mut lens = [0; 3];
            for &b in order.as_bytes() {
                if b.is_ascii_lowercase() {
                    assert!(!seen[(b - b'a') as usize]);
                    seen[(b - b'a') as usize] = true;

                    let pallet_count = count / 50;
                    if pallet_count != 0 {
                        lens[2] += 3 * pallet_count;
                    }

                    count %= 50;
                    let box_count = count / 10;
                    if box_count != 0 {
                        lens[1] += 3 * box_count;
                    }

                    count %= 10;
                    if count != 0 {
                        lens[0] += count + 2;
                    }

                    unsafe {
                        container_counts.push_unchecked((b, (count, box_count, pallet_count)));
                    }

                    count = 0;
                } else {
                    count *= 10;
                    count += (b - b'0') as usize;
                }
            }

            let cap = *lens.iter().max().unwrap();

            let mut res = (
                String::with_capacity(cap),
                String::with_capacity(cap),
                String::with_capacity(cap),
            );

            unsafe {
                for _ in 0..cap - lens[0] {
                    res.0.push_unchecked('-');
                }
                for _ in 0..cap - lens[1] {
                    res.1.push_unchecked('-');
                }
                for _ in 0..cap - lens[2] {
                    res.2.push_unchecked('-');
                }
            }

            for (ch, counts) in container_counts {
                unsafe {
                    if counts.0 != 0 {
                        res.0.push_unchecked('(');
                        for _ in 0..counts.0 {
                            res.0.as_mut_vec().push_unchecked(ch);
                        }
                        res.0.push_unchecked(')');
                    }
                    if counts.1 != 0 {
                        for _ in 0..counts.1 {
                            res.1.push_unchecked('{');
                            res.1.as_mut_vec().push_unchecked(ch);
                            res.1.push_unchecked('}');
                        }
                    }
                    if counts.2 != 0 {
                        for _ in 0..counts.2 {
                            res.2.push_unchecked('[');
                            res.2.as_mut_vec().push_unchecked(ch);
                            res.2.push_unchecked(']');
                        }
                    }
                }
            }

            res
        })
        .collect()
}
