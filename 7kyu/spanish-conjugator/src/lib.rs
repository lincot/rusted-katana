//! <https://www.codewars.com/kata/5a81b78d4a6b344638000183/train/rust>

use hashbrown::HashMap;
use unchecked::{PushStrUnchecked, PushUnchecked};

pub fn conjugate(verb: &str) -> HashMap<String, Vec<String>> {
    assert!(verb.len() >= 2);

    let base = unsafe { verb.get_unchecked(..verb.len() - 2) };

    let mut f1 = String::with_capacity(base.len() + "o".len());
    unsafe {
        f1.push_str_unchecked(base);
        f1.push_unchecked('o');
    }

    let forms = match verb.as_bytes()[verb.len() - 2] {
        b'a' => unsafe {
            let mut f2 = String::with_capacity(base.len() + "as".len());
            f2.push_str_unchecked(base);
            f2.push_str_unchecked("as");
            let mut f3 = String::with_capacity(base.len() + "a".len());
            f3.push_str_unchecked(base);
            f3.push_str_unchecked("a");
            let mut f4 = String::with_capacity(base.len() + "amos".len());
            f4.push_str_unchecked(base);
            f4.push_str_unchecked("amos");
            let mut f5 = String::with_capacity(base.len() + "áis".len());
            f5.push_str_unchecked(base);
            f5.push_str_unchecked("áis");
            let mut f6 = String::with_capacity(base.len() + "an".len());
            f6.push_str_unchecked(base);
            f6.push_str_unchecked("an");

            vec![f1, f2, f3, f4, f5, f6]
        },
        b'e' => unsafe {
            let mut f2 = String::with_capacity(base.len() + "es".len());
            f2.push_str_unchecked(base);
            f2.push_str_unchecked("es");
            let mut f3 = String::with_capacity(base.len() + "e".len());
            f3.push_str_unchecked(base);
            f3.push_str_unchecked("e");
            let mut f4 = String::with_capacity(base.len() + "emos".len());
            f4.push_str_unchecked(base);
            f4.push_str_unchecked("emos");
            let mut f5 = String::with_capacity(base.len() + "éis".len());
            f5.push_str_unchecked(base);
            f5.push_str_unchecked("éis");
            let mut f6 = String::with_capacity(base.len() + "en".len());
            f6.push_str_unchecked(base);
            f6.push_str_unchecked("en");

            vec![f1, f2, f3, f4, f5, f6]
        },
        b'i' => unsafe {
            let mut f2 = String::with_capacity(base.len() + "es".len());
            f2.push_str_unchecked(base);
            f2.push_str_unchecked("es");
            let mut f3 = String::with_capacity(base.len() + "e".len());
            f3.push_str_unchecked(base);
            f3.push_str_unchecked("e");
            let mut f4 = String::with_capacity(base.len() + "imos".len());
            f4.push_str_unchecked(base);
            f4.push_str_unchecked("imos");
            let mut f5 = String::with_capacity(base.len() + "ís".len());
            f5.push_str_unchecked(base);
            f5.push_str_unchecked("ís");
            let mut f6 = String::with_capacity(base.len() + "en".len());
            f6.push_str_unchecked(base);
            f6.push_str_unchecked("en");

            vec![f1, f2, f3, f4, f5, f6]
        },
        _ => panic!(),
    };

    [(verb.into(), forms)].into()
}
