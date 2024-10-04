//! <https://www.codewars.com/kata/59d727d40e8c9dd2dd00009f/train/rust>

use digital::{MaxLenBase10, Next2Digits, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn balance(book: &str) -> String {
    let cap = book.len()
        + "Original Balance: \nTotal expense  \nAverage expense  ".len()
        + 3
        + 2 * (u64::MAX_LEN_BASE10 + 3)
        + (1 + 3 + 1 + 1 + " Balance ".len() + u64::MAX_LEN_BASE10 + 3)
            * bytecount::count(book.as_bytes(), b'\n');
    let mut res = String::with_capacity(cap);

    unsafe {
        let balance_pos = book
            .as_bytes()
            .iter()
            .position(|&b| b.is_ascii_digit())
            .unwrap();

        let dot_pos = book.as_bytes().iter().position(|&b| b == b'.').unwrap();
        let mut balance = 100
            * book
                .get_unchecked(balance_pos..dot_pos)
                .parse::<u64>()
                .unwrap()
            + 10 * (book.as_bytes()[dot_pos + 1] - b'0') as u64;
        let mut end_pos = if book.as_bytes()[dot_pos + 2].is_ascii_digit() {
            balance += (book.as_bytes()[dot_pos + 2] - b'0') as u64;
            dot_pos + 3
        } else {
            dot_pos + 2
        };
        let original_balance = balance;
        res.push_str_unchecked("Original Balance: ");
        write_with_cents(&mut res, original_balance);

        let mut checks_count = 0;

        loop {
            let Some(start_pos) = book
                .as_bytes()
                .get_unchecked(end_pos..)
                .iter()
                .position(|&b| b.is_ascii_digit())
                .map(|pos| pos + end_pos)
            else {
                break;
            };

            res.push_unchecked('\n');

            let check_number = &book[start_pos..start_pos + 3];
            res.push_str_unchecked(check_number);
            res.push_unchecked(' ');

            let category_start_pos = (start_pos + 3)
                + book
                    .as_bytes()
                    .get_unchecked(start_pos + 3..)
                    .iter()
                    .position(|&b| b.is_ascii_alphabetic())
                    .unwrap();
            let category_end_pos = (category_start_pos + 1)
                + book
                    .as_bytes()
                    .get_unchecked(category_start_pos + 1..)
                    .iter()
                    .position(|&b| !b.is_ascii_alphabetic())
                    .unwrap();
            let category = book.get_unchecked(category_start_pos..category_end_pos);
            res.push_str_unchecked(category);
            res.push_unchecked(' ');

            let digit_pos = category_end_pos
                + book
                    .as_bytes()
                    .get_unchecked(category_end_pos..)
                    .iter()
                    .position(|&b| b.is_ascii_digit())
                    .unwrap();
            let dot_pos = digit_pos
                + book
                    .as_bytes()
                    .get_unchecked(digit_pos..)
                    .iter()
                    .position(|&b| b == b'.')
                    .unwrap();
            let mut price = 100
                * book
                    .get_unchecked(digit_pos..dot_pos)
                    .parse::<u64>()
                    .unwrap();
            if book.as_bytes()[dot_pos + 1].is_ascii_digit() {
                price += 10 * (book.as_bytes()[dot_pos + 1] - b'0') as u64;
            }
            let mut should_break = false;
            if book.len() >= dot_pos + 3 {
                if book.as_bytes()[dot_pos + 2].is_ascii_digit() {
                    price += (book.as_bytes()[dot_pos + 2] - b'0') as u64;
                    end_pos = dot_pos + 3;
                } else {
                    end_pos = dot_pos + 2;
                }
            } else {
                should_break = true;
            }

            balance -= price;
            checks_count += 1;
            write_with_cents(&mut res, price);
            res.push_str_unchecked(" Balance ");
            write_with_cents(&mut res, balance);

            if should_break {
                break;
            }
        }

        let total_expense = original_balance - balance;
        res.push_str_unchecked("\nTotal expense  ");
        write_with_cents(&mut res, total_expense);
        res.push_str_unchecked("\nAverage expense  ");
        let average = (total_expense + checks_count / 2) / checks_count;
        write_with_cents(&mut res, average);
    }

    res
}

unsafe fn write_with_cents(s: &mut String, mut amount: u64) {
    let last_2 = amount.next_2_digits(false).unwrap_or_else(|| {
        let d = amount;
        amount = 0;
        [b'0', d as u8 + b'0']
    });
    s.write_num_unchecked(amount, 10, false, false);
    s.as_mut_vec().push_unchecked(b'.');
    s.as_mut_vec().push_unchecked(last_2[0]);
    s.as_mut_vec().push_unchecked(last_2[1]);
}
