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
        unimplemented!(
            "Return the protein name for a '{codon}' codon or None, if codon string is invalid"
        );
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        unimplemented!("Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid");
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo::new(pairs)
}
