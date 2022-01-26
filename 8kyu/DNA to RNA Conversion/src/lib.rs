//! <https://www.codewars.com/kata/5556282156230d0e5e000089/train/rust>

pub fn dna_to_rna(dna: &str) -> String {
    let res = dna
        .bytes()
        .map(|c| if c == b'T' { b'U' } else { c })
        .collect();

    unsafe { String::from_utf8_unchecked(res) }
}
