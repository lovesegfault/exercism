use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

#[inline(always)]
fn is_valid_nucleotide(nucleotide: char) -> bool {
    VALID_NUCLEOTIDES.contains(&nucleotide)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;

    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !is_valid_nucleotide(c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::with_capacity(dna.len() / VALID_NUCLEOTIDES.len());
    VALID_NUCLEOTIDES
        .iter()
        .map(|n| counts.insert(*n, 0))
        .for_each(|_| ());

    for nucleotide in dna.chars() {
        if !is_valid_nucleotide(nucleotide) {
            return Err(nucleotide);
        }
        *counts.get_mut(&nucleotide).unwrap() += 1;
    }

    Ok(counts)
}
