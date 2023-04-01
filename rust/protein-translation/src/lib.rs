use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct CodonsInfo<'a> {
    codons: BTreeMap<&'a str, &'a str>,
    acids: BTreeMap<&'a str, Vec<&'a str>>,
}

impl<'a> CodonsInfo<'a> {
    pub fn new(pairs: Vec<(&'a str, &'a str)>) -> Self {
        let mut codons = BTreeMap::new();
        let mut acids = BTreeMap::new();
        for (codon, name) in pairs {
            codons.insert(codon, name);
            acids.entry(name).or_insert_with(Vec::new).push(codon);
        }
        CodonsInfo { codons, acids }
    }

    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result = Vec::new();
        for codon in rna.as_bytes().chunks(3) {
            let codon = std::str::from_utf8(codon).ok()?;
            if let Some(name) = self.name_for(codon) {
                if name == "stop codon" {
                    return Some(result);
                }
                result.push(name);
            } else {
                return None;
            }
        }
        Some(result)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo::new(pairs)
}
