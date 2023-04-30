const SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

#[derive(Debug)]
pub enum Error {
    InvalidTonic,
    InvalidInterval,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scale(Vec<String>);

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        unimplemented!("Construct a new scale with tonic {tonic} and intervals {intervals}")
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        unimplemented!("Construct a new chromatic scale with tonic {tonic}")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}
