//! <https://www.codewars.com/kata/56a32dd6e4f4748cc3000006/train/rust>

use core::mem::{transmute, MaybeUninit};

pub fn mean(town: &str, strng: &str) -> f64 {
    let Some(records) = get_city_records(strng, town) else {
        return -1.;
    };
    records.iter().sum::<f64>() / records.len() as f64
}

pub fn variance(town: &str, strng: &str) -> f64 {
    let Some(records) = get_city_records(strng, town) else {
        return -1.;
    };
    let mean = records.iter().sum::<f64>() / records.len() as f64;
    records.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / records.len() as f64
}

fn get_city_records(data: &str, town: &str) -> Option<[f64; 12]> {
    let line = data
        .as_bytes()
        .split(|&x| x == b'\n')
        .find(|line| line.starts_with(town.as_bytes()))?;

    let mut chunks = line[town.len()..].split(|&x| x == b' ');
    chunks.next();

    let mut res = [MaybeUninit::uninit(); 12];
    for (i, r) in res.iter_mut().enumerate() {
        let chunk = chunks.next()?;
        let num_end = if i == 11 {
            chunk.len()
        } else {
            chunk.iter().position(|&x| x == b',')?
        };
        let num_str = unsafe { core::str::from_utf8_unchecked(&chunk[..num_end]) };
        r.write(num_str.parse().ok()?);
    }
    Some(unsafe { transmute::<[MaybeUninit<f64>; 12], [f64; 12]>(res) })
}
