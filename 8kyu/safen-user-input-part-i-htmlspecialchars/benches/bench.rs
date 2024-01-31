#![feature(test)]

extern crate test;
use safen_user_input_part_i_htmlspecialchars::html_special_chars;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| html_special_chars(black_box("<script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script><script>alert('Website Hacked!');</script>")));
}
