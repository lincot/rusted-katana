#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use where_my_anagrams_at::anagrams;

#[bench]
fn bench(bencher: &mut Bencher) {
    let word = black_box("невозможно");
    let words = [
        "ннжомезвоо".into(),
        "жеОоознвнм".into(),
        "зеовннможо...".into(),
        "озмОнжвоне".into(),
        "зжеОоонвмн".into(),
        "Оноженмзво".into(),
        "онзовжонем".into(),
        "нвезжооном...".into(),
        "енжвоомонз...".into(),
        "немооонзжв...".into(),
        "енОвмжонзо".into(),
        "ензжнвоомо...".into(),
        "еомжовнзно".into(),
        "зеонмоовжн".into(),
        "нонзооевмж".into(),
        "возооенжнм...".into(),
        "Ожоннемвоз".into(),
        "жоеномознв".into(),
        "овемнзжноо".into(),
        "ожеознонвм...".into(),
        "оннемвожзо".into(),
        "знноежомво...".into(),
        "жвОномонзе".into(),
        "овноенжзом".into(),
        "вооонжнмез...".into(),
        "нОнмжзеоов".into(),
        "вмознжноое...".into(),
        "вОжоноемнз".into(),
        "монжнзвоео".into(),
        "оежзнОомнв".into(),
    ];
    let words = black_box(&words);
    bencher.iter(|| anagrams(word, words));
}
