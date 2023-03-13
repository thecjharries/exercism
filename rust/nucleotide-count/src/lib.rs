use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for c in dna.chars() {
        if !VALID_NUCLEOTIDES.contains(&c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    unimplemented!("How much of every nucleotide type is contained inside DNA string '{dna}'?");
}
