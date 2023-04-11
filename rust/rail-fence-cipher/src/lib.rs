use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub struct RailFence(u32);

impl RailFence {
    pub fn explode(text: &str) -> Vec<&str> {
        text.graphemes(true).collect()
    }

    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![String::new(); self.0 as usize];
        let exploded = RailFence::explode(text);
        let mut rail: u32 = 0;
        let mut direction: i32 = 1;
        for c in exploded {
            rails[rail as usize].push_str(c);
            rail = (rail as i32 + direction) as u32;
            if rail == 0 || rail == self.0 - 1 {
                direction *= -1;
            }
        }
        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {cipher}")
    }
}
