#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_fusc_function_part_2::fusc;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| fusc(black_box("34938200355436519500894816632962340253735841720231926891980056097562505970566922564302329749568747509112253761562638537889389828876875528865509182895341310407469618807513511356421149839845190201680990577576411106794069057441057038710516346594726562844199927044270091660659198231458566462239269248837754447203470945803231542566831846393169814050631673708101872950775179831935861304206431765487250074181082726886012555801435662108631375245666613768539783760101374018903958505936060254750609582748067260010366955964504679824630279311408683201204946512876958905302633438190481666637855600154388251334100446".parse().unwrap())));
}
