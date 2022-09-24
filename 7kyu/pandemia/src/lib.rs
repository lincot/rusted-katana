//! <https://www.codewars.com/kata/5e2596a9ad937f002e510435/train/rust>

#![no_std]

pub fn infected(s: &str) -> f64 {
    let mut total_population = 0usize;
    let mut total_infected = 0usize;

    let mut continent_population = 0usize;
    let mut continent_infected = false;

    for b in s.bytes() {
        if b == b'X' {
            total_population += continent_population;
            if continent_infected {
                total_infected += continent_population;
            }

            continent_population = 0;
            continent_infected = false;
        } else {
            continent_population += 1;
            if b == b'1' {
                continent_infected = true;
            }
        }
    }

    total_population += continent_population;
    if continent_infected {
        total_infected += continent_population;
    }

    if total_population == 0 {
        0.
    } else {
        100. * total_infected as f64 / total_population as f64
    }
}
