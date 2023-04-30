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
    #[cfg(not(tarpaulin_include))]
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let mut scale = Vec::new();
        let notes = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
            | "d#" => SHARPS,
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => FLATS,
            _ => return Err(Error::InvalidTonic),
        };
        let mut index = notes
            .iter()
            .position(|&x| x.to_uppercase() == tonic.to_uppercase())
            .unwrap();
        scale.push(notes[index].to_string());
        for interval in intervals.chars() {
            index += match interval {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => return Err(Error::InvalidInterval),
            };
            scale.push(notes[index % 12].to_string());
        }
        Ok(Scale(scale))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}
