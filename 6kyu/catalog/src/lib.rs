//! <https://www.codewars.com/kata/59d9d8cb27ee005972000045/train/rust>

use unchecked_std::prelude::*;

pub fn catalog(s: &str, article: &str) -> String {
    let mut res = String::with_capacity(s.len());

    for a in s.split("\n\n") {
        let article_len = a.as_bytes()["<prod><name>".len()..]
            .iter()
            .position(|&b| b == b'<')
            .unwrap();
        let cur_article =
            unsafe { a.get_unchecked("<prod><name>".len().."<prod><name>".len() + article_len) };

        if !cur_article.contains(article) {
            continue;
        }
        unsafe {
            res.push_str_unchecked(cur_article);
            res.push_str_unchecked(" > prx: $");
        }

        let prx_len = a.as_bytes()["<prod><name></name><prx>".len() + article_len..]
            .iter()
            .position(|&b| b == b'<')
            .unwrap();
        unsafe {
            res.push_str_unchecked(a.get_unchecked(
                "<prod><name></name><prx>".len() + article_len
                    .."<prod><name></name><prx>".len() + article_len + prx_len,
            ));
            res.push_str_unchecked(" qty: ");
        }

        let qty_len = a.as_bytes()
            ["<prod><name></name><prx></prx><qty>".len() + article_len + prx_len..]
            .iter()
            .position(|&b| b == b'<')
            .unwrap();
        unsafe {
            res.push_str_unchecked(a.get_unchecked(
                "<prod><name></name><prx></prx><qty>".len() + article_len + prx_len
                    .."<prod><name></name><prx></prx><qty>".len() + article_len + prx_len + qty_len,
            ));
            res.push_unchecked('\n');
        }
    }

    if res.is_empty() {
        "Nothing".into()
    } else {
        res.pop();
        res
    }
}
