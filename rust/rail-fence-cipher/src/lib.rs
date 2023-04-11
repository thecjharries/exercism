#[derive(Debug, PartialEq)]
pub struct RailFence(u32);

impl RailFence {
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
