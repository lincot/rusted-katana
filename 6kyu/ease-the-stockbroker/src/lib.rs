//! <https://www.codewars.com/kata/54de3257f565801d96001200/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn balance_statement(lst: &str) -> String {
    let mut buy = 0.;
    let mut sell = 0.;
    let mut badly_formed = Vec::new();
    let mut start = 0;
    if !lst.is_empty() {
        for i in 0..lst.len() - 1 {
            if &lst.as_bytes()[i..i + 2] == b", " {
                let chunk = unsafe { lst.get_unchecked(start..i) };
                match parse_simple_order(chunk) {
                    None => badly_formed.push(chunk),
                    Some(Ok(n)) => buy += n,
                    Some(Err(n)) => sell += n,
                }
                start = i + 2;
            }
        }
    }
    if start != 0 {
        let chunk = unsafe { lst.get_unchecked(start..) };
        match parse_simple_order(chunk) {
            None => badly_formed.push(chunk),
            Some(Ok(n)) => buy += n,
            Some(Err(n)) => sell += n,
        }
    }

    let mut cap = "Buy:  Sell: ".len() + 2 * u64::MAX_LEN_BASE10;
    if !badly_formed.is_empty() {
        cap += "; Badly formed : ".len()
            + usize::MAX_LEN_BASE10
            + badly_formed.iter().map(|s| s.len()).sum::<usize>()
            + " ;".len() * badly_formed.len();
    }
    let mut res = String::with_capacity(cap);
    unsafe {
        res.push_str_unchecked("Buy: ");
        res.write_num_unchecked(weird_round(buy), 10, false, false);
        res.push_str_unchecked(" Sell: ");
        res.write_num_unchecked(weird_round(sell), 10, false, false);
        if !badly_formed.is_empty() {
            res.push_str_unchecked("; Badly formed ");
            res.write_num_unchecked(badly_formed.len(), 10, false, false);
            res.push_str_unchecked(": ");
            for bad in badly_formed {
                res.push_str_unchecked(bad);
                res.push_str_unchecked(" ;");
            }
        }
    }
    res
}

fn parse_simple_order(s: &str) -> Option<Result<f64, f64>> {
    let space_pos = s.bytes().position(|b| b == b' ')?;
    let mut i = space_pos + 1;
    unsafe {
        let space_pos = s
            .as_bytes()
            .get_unchecked(i..)
            .iter()
            .position(|&b| b == b' ')?;
        let quantity: u32 = s.get_unchecked(i..i + space_pos).parse().ok()?;
        i += space_pos + 1;
        let space_pos = s.get_unchecked(i..).bytes().position(|b| b == b' ')?;
        let price_s = s.get_unchecked(i..i + space_pos);

        if !price_s.as_bytes().contains(&b'.') {
            return None;
        }
        let price: f64 = price_s.parse().ok()?;
        let total_price = price * quantity as f64;

        match s.as_bytes().get(i + space_pos + 1)? {
            b'B' => Some(Ok(total_price)),
            b'S' => Some(Err(total_price)),
            _ => None,
        }
    }
}

fn weird_round(n: f64) -> u64 {
    if n == 0. {
        return 0;
    }
    let bits = n.to_bits();
    if bits & (1 << (bits.trailing_zeros() + 1)) == 0 {
        (n + (0.5 - 1e-9)) as _
    } else {
        (n + 0.5) as _
    }
}
