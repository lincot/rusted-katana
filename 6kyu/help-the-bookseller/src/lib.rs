//! <https://www.codewars.com/kata/54dc6f5a224c26032800005c/train/rust>

use my_prelude::prelude::*;

pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return String::new();
    }

    let cap = (4 + 20 + "( : ) - ".len()) * list_cat.len();
    let mut res = String::with_capacity(cap);

    let mut list_cat = list_cat.into_iter();
    if let Some(cat) = list_cat.next() {
        let cat = cat.chars().next().unwrap();

        let mut sum = 0;
        for &art in &list_art {
            if art.starts_with(cat) {
                let space_pos = art.bytes().position(|b| b == b' ').unwrap();
                sum += unsafe { art.get_unchecked(space_pos + 1..) }
                    .parse::<usize>()
                    .unwrap();
            }
        }

        unsafe {
            res.push_unchecked('(');
            res.push_unchecked(cat);
            res.push_str_unchecked(" : ");
            res.write_num_unchecked(sum);
            res.push_unchecked(')');
        }
    }
    for cat in list_cat {
        let cat = cat.chars().next().unwrap();

        let mut sum = 0;
        for &art in &list_art {
            if art.starts_with(cat) {
                let space_pos = art.bytes().position(|b| b == b' ').unwrap();
                sum += unsafe { art.get_unchecked(space_pos + 1..) }
                    .parse::<usize>()
                    .unwrap();
            }
        }

        unsafe {
            res.push_str_unchecked(" - (");
            res.push_unchecked(cat);
            res.push_str_unchecked(" : ");
            res.write_num_unchecked(sum);
            res.push_unchecked(')');
        }
    }

    res
}