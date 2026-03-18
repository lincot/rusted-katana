#![feature(test)]

extern crate test;
use replace_with_alphabet_position::alphabet_position;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const TEXT: &str = "Many recent philosophers have also diverged from what some would describe as ideals characteristic of traditional Platonism. Friedrich Nietzsche notoriously attacked Plato's \"idea of the good itself\" along with many fundamentals of Christian morality, which he interpreted as \"Platonism for the masses\" in Beyond Good and Evil (1886). Martin Heidegger argued against Plato's alleged obfuscation of Being in his incomplete tome, Being and Time (1927). Karl Popper argued in the first volume of The Open Society and Its Enemies (1945) that Plato's proposal for a \"utopian\" political regime in the Republic was prototypically totalitarian; this has been disputed. Edmund Gettier famously demonstrated the Gettier problem for the justified true belief account of knowledge.";

    bencher.iter(|| alphabet_position(black_box(TEXT)));
}
