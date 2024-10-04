#![feature(test)]

extern crate test;
use meeting::meeting;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    const S: &str = "Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn";

    bencher.iter(|| meeting(black_box(S)));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    const S: &str = "Алексис:Валь;Джон:Белл;Виктория:Шварц;Абба:Дорни;Грейс:Мета;Энн:Арно;Мэдисон:СТЭН;Алекс:Корнуэлл;Льюис:Керн;Меган:Стэн;Алекс:Корн";

    bencher.iter(|| meeting(black_box(S)));
}
