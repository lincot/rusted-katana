#![feature(test)]

extern crate test;
use shortest_word::find_short;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_short(black_box("В 367 или 366 г. до н. э., после смерти Дионисия Старшего, его сын и преемник Дионисий Младший под влиянием своего дяди Диона (с которым Платон подружился ещё в первое своё посещение Сиракуз в Сицилии) приглашает философа, обещая стать его верным учеником. Сначала мечта Платона о юном тиране, управляющем обществом под руководством истинного философа, как будто сбывается. Но скоро Дионисию надоедает философское наблюдение; после своего разрыва с Дионом он начинает негативно относиться к Платону и выгоняет его[8]. В 361 году через пифагорейца Архита, Дионисий Младший снова призывает Платона, обещая ему помириться с Дионом, и снова его обманывает, так что 70-летний Платон принуждён бежать из Сиракуз[8]. Предполагается, что Аристотель вошёл в Академию до возвращения Платона.")));
}
