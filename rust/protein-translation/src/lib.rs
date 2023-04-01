use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct CodonsInfo<'a> {
    codons: BTreeMap<&'a str, &'a str>,
    acids: BTreeMap<&'a str, Vec<&'a str>>,
}

impl<'a> CodonsInfo<'a> {
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
    unimplemented!("Construct a new CodonsInfo struct from given pairs: {pairs:?}");
}
