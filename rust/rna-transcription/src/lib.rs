#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<char>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<char>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut characters = Vec::new();
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'T' => characters.push(c),
                _ => return Err(i),
            }
        }
        Ok(Dna(characters))
    }

    pub fn into_rna(self) -> Rna {
        unimplemented!("Transform Dna {self:?} into corresponding Rna");
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut characters = Vec::new();
        for (i, c) in rna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'U' => characters.push(c),
                _ => return Err(i),
            }
        }
        Ok(Rna(characters))
    }
}
