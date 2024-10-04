#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use tims_fruit_shop_challenge::fruit_pack;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        fruit_pack(black_box(if cfg!(miri) {
            &["53a188b180s127d151z144x33m17u116i190n", "29l17x"]
        } else {
            &[
                "53a188b180s127d151z144x33m17u116i190n",
                "91r96b128w64d36u34z62g190h91t1j49k127l10m54n165q",
                "139a171z48y104u156e100r6x139s110i105t23m85l",
                "82l148b86c105d65p54x171g128h84w133o169k",
                "44a25b66c86d135e94f11g140h91i4j127k80l11w51z151o150u86q191r154y53v",
                "29l17x",
                "89w74r111n154d",
                "109a170k111u106q64e192f116n57v171j",
                "90t48l16x",
                "120v157b121w135r188p",
                "141x23b179r73d114e181f20u75t33i147j185k52l39m137s118o84y5q",
                "27a93t128c139w78e179f179g126z153y78j2k20l89x130u131o47p",
                "65u5r136s168d100e154f121y185h",
                "191a71b171t153z91y8f98g165h186u17j114k71l141m47n46o50p113q155r",
                "46a4b161c36r138t169f78v130h33i78j144s117l119m197x34o130p",
                "47e8v90t53l",
                "76a22b126c107d53z170q182g120h134i40t53v60l109m141n94o162p",
                "142w76b153p101l178e110f46x32h174i83j",
            ]
        }))
    });
}
