//! <https://www.codewars.com/kata/5556282156230d0e5e000089/train/rust>

pub fn dna_to_rna(dna: &str) -> String {
    let res = dna
        .as_bytes()
        .iter()
        .map(|&b| if b == b'T' { b'U' } else { b })
        .collect();
    unsafe { String::from_utf8_unchecked(res) }
}
