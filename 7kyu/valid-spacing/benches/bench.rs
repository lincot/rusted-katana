#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use valid_spacing::valid_spacing;

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    const S: &str = "Many recent philosophers have also diverged from what some would describe as ideals characteristic of traditional Platonism. Friedrich Nietzsche notoriously attacked Plato's \"idea of the good itself\" along with many fundamentals of Christian morality, which he interpreted as \"Platonism for the masses\" in Beyond Good and Evil (1886). Martin Heidegger argued against Plato's alleged obfuscation of Being in his incomplete tome, Being and Time (1927). Karl Popper argued in the first volume of The Open Society and Its Enemies (1945) that Plato's proposal for a \"utopian\" political regime in the Republic was prototypically totalitarian; this has been disputed. Edmund Gettier famously demonstrated the Gettier problem for the justified true belief account of knowledge.";

    bencher.iter(|| valid_spacing(black_box(S)));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    const S: &str = "В 367 или 366 г. до н. э., после смерти Дионисия Старшего, его сын и преемник Дионисий Младший под влиянием своего дяди Диона (с которым Платон подружился ещё в первое своё посещение Сиракуз в Сицилии) приглашает философа, обещая стать его верным учеником. Сначала мечта Платона о юном тиране, управляющем обществом под руководством истинного философа, как будто сбывается. Но скоро Дионисию надоедает философское наблюдение; после своего разрыва с Дионом он начинает негативно относиться к Платону и выгоняет его. В 361 году через пифагорейца Архита, Дионисий Младший снова призывает Платона, обещая ему помириться с Дионом, и снова его обманывает, так что 70-летний Платон принуждён бежать из Сиракуз. Предполагается, что Аристотель вошёл в Академию до возвращения Платона.";

    bencher.iter(|| valid_spacing(black_box(S)));
}
