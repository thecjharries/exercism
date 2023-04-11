use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub struct RailFence(u32);

impl RailFence {
    pub fn explode(text: &str) -> Vec<&str> {
        test.graphemes(true).collect()
    }

    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        unimplemented!("Encode this text: {text}")
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {cipher}")
    }
}
